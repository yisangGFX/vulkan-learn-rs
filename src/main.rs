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
    VulkanLibrary, device::DeviceExtensions, 
};

struct QueueFamilyIndices {
    graphics_family: i32,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self { graphics_family: -1 }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family >= 0
    }
}
#[allow(unused)]
struct  HelloTriangleApplication {
    event_loop: Option<EventLoop<()>>,
    instance: Option<Arc<Instance>>,
    surface: Arc<Surface>,
    physical_device_index: usize,   // can't store PhysicalDevice directly (lifetime issues)
}

impl HelloTriangleApplication {
    pub fn initialize() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Vulkan-learn-rs").build(&event_loop).unwrap();
        let instance = Self::create_instance();
        let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();
        
        let physical_device_index = Self::pick_physical_device(&instance);
        
        Self {
            event_loop: Some(event_loop),
            instance: Some(instance),
            surface: surface,
            physical_device_index: physical_device_index,
        }
    }

    fn create_instance() -> Arc<Instance> {
        let library = VulkanLibrary::new().unwrap();

        let required_extensions = vulkano_win::required_extensions(&library);
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                enumerate_portability: true,
                ..Default::default()
            },
        )
        .unwrap();
        instance
    }

    fn pick_physical_device(instance: &Arc<Instance>) -> usize {
        let mut device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
    }

    fn main_loop(&mut self) {
        if let Some(event_loop) = self.event_loop.take() { // take the event_loop out
            event_loop.run(move |event, _, control_flow| {
                control_flow.set_poll();
                control_flow.set_wait();
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        println!("The close button was pressed; stopping");
                        control_flow.set_exit();
                    },
                    Event::RedrawRequested(_) => {
    
                    },
                    _ => ()
                }
            });
        }
    }
}

fn main() {
    let mut app = HelloTriangleApplication::initialize();
    app.main_loop();
}
