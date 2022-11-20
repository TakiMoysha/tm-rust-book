use std::io::stdin;
use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    instance::{ Instance, InstanceCreateInfo },
    device::{
        Device, DeviceExtensions, DeviceCreateInfo, physical::PhysicalDeviceType, QueueCreateInfo
    }, swapchain::{Swapchain, SwapchainCreateInfo}, image::ImageUsage,
};
use winit::{
    event::{Event, WindowEvent, },
    event_loop::{EventLoop, ControlFlow, },
    window::{Window, WindowBuilder, },
};
use vulkano_win::VkSurfaceBuild;

fn select_device(vulkan_instance: Arc<Instance>) -> u32 {
    let vk_phys_devices = vulkan_instance.enumerate_physical_devices().unwrap();
    let vk_phys_devices_len = vk_phys_devices.len();
    if vk_phys_devices_len == 0 {
        panic!("Physical devices not founded, exit...");
    }
    println!("Physical devices founded:");
    for (index, device) in vk_phys_devices.enumerate() {
        println!(
            "\t{}) - {:?} - {:?}",
            index,
            device.properties().device_name,
            device.properties().device_type
        );
    }

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read you input");
    let user_input_index: u32 = match buf.trim().parse::<u32>() {
        Ok(parse_res) => {
            if (parse_res as usize) + 1 > vk_phys_devices_len {
                println!("Incorrect number, used default - 0");
                0
            } else {
                parse_res
            }
        }
        Err(error) => {
            eprintln!("Error {error}\nWrong input number, set default - 0");
            0
        }
    };

    // vk_phys_devices.position(predicate)
    0
}

fn main() {
    let library = VulkanLibrary::new().unwrap();
    let required_extensions = vulkano_win::required_extensions(&library);

    let instance = Instance::new(library, InstanceCreateInfo {
        enabled_extensions: required_extensions,
        enumerate_portability: true,
        ..Default::default()
    }).expect("Expect to create a vulkan instance.");

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Error while create surface");

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| { p.supported_extensions().contains(&device_extensions) })
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| {
            match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            }
        })
        .expect("No suitable physical device found");

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index, ..Default::default()
            }],

            ..Default::default()
        }
    ).expect("Error in init device");

    let queue = queues.next().unwrap();

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
        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
    }

    // let selected_device = select_device(instance);
    event_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}