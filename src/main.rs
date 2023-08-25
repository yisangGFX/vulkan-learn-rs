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
    VulkanLibrary, device::DeviceExtensions, library, 
};

fn main() {
    let library = VulkanLibrary::new().unwrap();
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

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Vulkan-learn-rs").build(&event_loop).unwrap();
    
    let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();
        
}
