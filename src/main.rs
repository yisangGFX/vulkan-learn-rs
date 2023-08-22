extern crate vulkano;
extern crate winit;

use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    swapchain::{Surface},
    VulkanLibrary, 
};

struct  HelloTriangleApplication {
    event_loop: Option<EventLoop<()>>,
    instance: Option<Arc<Instance>>,
}

impl HelloTriangleApplication {
    pub fn initialize() -> Self {
        let event_loop = Self::init_window();
        let instance = Self::create_instance();
        Self {
            event_loop: Some(event_loop),
            instance: Some(instance),
        }
    }

    fn init_window() -> EventLoop<()> {
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Vulkan learn rs")
            .build(&event_loop).unwrap();
        event_loop
    }

    fn create_instance() -> Arc<Instance> {
        let library = VulkanLibrary::new().unwrap();

        let required_extensions = vulkano_win::required_extensions(&library);
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .unwrap();
        instance
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
