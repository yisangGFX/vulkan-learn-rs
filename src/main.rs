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
    VulkanLibrary, device::{DeviceExtensions, physical::{self, PhysicalDeviceType}, QueueFlags}, library, 
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
    let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();
        
    // #4. Enumerate the physical devices
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, que_family_index) = instance
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
    )
}
