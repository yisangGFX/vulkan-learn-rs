extern crate vulkano;
extern crate winit;

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
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    VulkanLibrary, 
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    }, image::{self, ImageUsage},
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

   // #6. Swapchain
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

}
