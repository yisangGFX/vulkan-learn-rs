use std::sync::Arc;
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    swapchain::{
        acquire_next_image, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    VulkanLibrary, 
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            vertex_input::Vertex,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline,
    },
    image::{view::ImageView, ImageAccess, ImageUsage, SwapchainImage},
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryTypeFilter, MemoryUsage}, 
    render_pass::{Subpass, RenderPass, Framebuffer, FramebufferCreateInfo},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassContents, self,
    },
    sync::{self, GpuFuture},
};

fn main() {
    // #1. VulkanLibrary
    let library = VulkanLibrary::new().unwrap();

    // #2. Instance
    let require_extensions = vulkano_win::required_extensions(&library);
    let instance = Instance::new(
        library,
        InstanceCreateInfo { 
            application_name: Some("Learn Vulkan-rs".to_string()), 
            enabled_extensions: require_extensions,
            enumerate_portability: true,
            ..Default::default()
        },
    ).unwrap();

    // #3. Surface
    let event_loop = EventLoop::new();
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();
        
    // #4. Enumerate the physical devices
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
    .enumerate_physical_devices()
    .unwrap()
    .filter(|p| {
        p.supported_extensions().contains(&device_extensions)
    })
    .filter_map(|p|{
        p.queue_family_properties()
        .iter()
        .enumerate()
        .position(|(i,q)| {
            q.queue_flags.intersects(QueueFlags::GRAPHICS)
                && p.surface_support(i as u32, &surface)
                .unwrap_or(false)
        })
        .map(|i| (p, i as u32))
    })
    .min_by_key(|(p, _)|{
        match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        }
    })
    .expect("no suitable physical device found");

    // Some debug informations
    print!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    // #5. Logical device and queues
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo{
            enabled_extensions:device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .unwrap();

    let queue = queues.next().unwrap();

   // #6. Swapchain & image
   let (mut swapchain, images) = {
        let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .unwrap();

        let image_format = Some(
            device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0,
        );

        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo { 
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
   };

   // #7. Command buffer 
   // #7.1 Buffer
   #[derive(BufferContents, Vertex)]
   #[repr(C)]
   struct Vertex {
        #[format(R32G32_SFLOAT)]
        position: [f32; 2],
   }

   let vertices = [
        Vertex {
            position: [-0.5, -0.25],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.25, -0.1],
        },
   ];

   let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
   let vertex_buffer = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vertices,
   )
   .unwrap();

   // #7.2 Shader
   mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: r"
                #version 450

                layout(location = 0) 
                in vec2 position;

                void main(){
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
        }
   }
   mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: r"
                #version 450

                layout(location = 0) 
                out vec4 f_color;

                void main(){
                    f_color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            ",
        }
   }

   let vs = vs::load(device.clone()).unwrap();
   let fs = fs::load(device.clone()).unwrap();

   // #7.3 render pass
   let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
   )
   .unwrap();

   // #7.4 pipeline 
   let pipeline = GraphicsPipeline::start()
   .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
   .vertex_input_state(Vertex::per_vertex())
   .input_assembly_state(InputAssemblyState::new())
   .vertex_shader(vs.entry_point("main").unwrap(), ())
   .build(device.clone())
   .unwrap();

   // Dynamic viewpoint
   let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    // #7.5 FBO
    let mut frambuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    // allocating command buffer
    let command_buffer_allocator = 
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let mut recreate_swapchain = false;

    // store the submission of the previous frame here.
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
