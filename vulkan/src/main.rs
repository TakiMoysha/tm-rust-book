use std::sync::Arc;

use vulkano::instance::{Instance, InstanceCreateInfo, };
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use vulkano_win::VkSurfaceBuild;

fn run_windows_window(vulkan_instance: Arc<Instance>) {
    // create the actual window
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, vulkan_instance.clone())
        .unwrap();

    // run event loop
    event_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}


fn show_physical_devices(vulkan_instance: Arc<Instance>) {
    for device in vulkan_instance.enumerate_physical_devices().unwrap() {
        println!("Physical Device: {:?}", device.properties().device_name);
    }
}
fn main() {
    let library = vulkano::VulkanLibrary::new()
        .expect("no local Vulkan library/DLL");
    let required_extensions = vulkano_win::required_extensions(&library);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        }
    ).expect("Expect to create a vulkan instance, but too unlucky");

    show_physical_devices(instance);
    // run_windows_window(instance);
}
