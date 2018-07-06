#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::os::raw::c_char;
use std::ffi::*;
use std::fmt;

mod vk;
use vk::*;

mod libc;
use libc::*;

mod glfw;
use glfw::*;

// fn display_properties(value: &VkPhysicalDeviceProperties) {
//     println!("API version   : {:#?}", value.api_version);
//     println!("Driver version: {:#?}", value.driver_version);
//     println!("Vendor ID     : {:#?}", value.vendor_id);
//     println!("Device ID     : {:#?}", value.device_id);
//     println!("Device type   : {:#?}", value.device_type);
//     println!("Device name   : {:#?}", value.device_name);
// }

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;

fn main() {
    let instance = VkInstance::new(&VkInstanceCreateInfo {
        flags: VkInstanceCreateFlags { },
        application_info: VkApplicationInfo {
            application_name: String::from("foo"),
            application_version: [0, 1, 0],
            engine_name: String::from("engine"),
            engine_version: [0, 1, 0],
            api_version: [1, 0, 0]
        },
        enabled_layer_names: Vec::new(),
        enabled_extension_names: Vec::new()
    }).expect("Failed to create instance");

    let instance_supported_extensions : Vec<String> = instance
        .get_supported_extensions()
        .expect("Failed to retrieve instance extensions")
        .into_iter().map(|ext| ext.extension_name).collect();
    
    let physical_devices = instance.get_physical_devices().expect("Failed to retrieve physical devices");
    let physical_device = &physical_devices[0];

    println!("{:#?}", instance_supported_extensions);

    // let glfw = GlfwInstance::new();
    // let required_extensions = glfw.get_required_vulkan_extensions().unwrap();
    // let window = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vulkan");
    // let instance = VkInstance::create(&VkInstanceCreateInfo {
    //     application_info: Default::default(),
    //     enabled_layers: Vec::new(),
    //     enabled_extensions: required_extensions
    // }).expect("Failed to create VkInstance");

    // let surface = instance.create_surface_from_glfw(&window).expect("Failed to create VkSurface");

    // let physical_devices = instance.get_physical_devices().expect("Failed to retrieve physical devices");
    // let physical_device = &physical_devices[0];
    // let features = physical_device.get_features();
    // let properties = physical_device.get_properties();
    // let queue_families = physical_device.get_queue_families();
    // let queue_family = &queue_families[0];
    // let device = physical_device.create_logical_device(&Default::default()).expect("Failed to initialize Device");
    // let queue = device.get_queue(0, 0);
    // let buffer = device.create_buffer(&VkBufferCreateInfo {
    //     size: 128,
    //     flags: VkBufferCreateFlags::none(),
    //     usage: VkBufferUsageFlags::all(),
    //     sharing_mode: VkSharingMode::Exclusive,
    //     queue_families: Vec::new()
    // }).expect("Failed to create Buffer");

    // let is_surface_supported = physical_device.does_support_surface(0, &surface).unwrap();
    // let instance_supported_extensions : Vec<String> = instance.get_supported_extensions().unwrap().into_iter().map(|ext| ext.extension_name).collect();
    // let device_supported_extensions : Vec<String> = physical_device.get_supported_extensions().expect("Failed to retrieve device extensions").into_iter().map(|ext| ext.extension_name).collect();
    // let capabilities = physical_device.get_surface_capabilities(&surface).expect("Failed to retrieve surface capabilities");
    // let surface_present_modes = physical_device.get_surface_present_modes(&surface).expect("Failed to retrieve present modes");

    // let swapchain = device.create_swapchain(&VkSwapchainCreateInfo {
    //     surface: &surface,
    //     min_image_count: 2,
    //     image_format: VkFormat::B8G8R8A8_UNORM,
    //     image_color_space: VkColorSpace::SrgbNonlinear,
    //     image_extent: VkExtent2D {
    //         width: WINDOW_WIDTH,
    //         height: WINDOW_HEIGHT
    //     },
    //     image_array_layers: 1,
    //     image_usage: VkImageUsageFlags {
    //         color_attachment: true,
    //         ..VkImageUsageFlags::none()
    //     },
    //     image_sharing_mode: VkSharingMode::Exclusive,
    //     queue_family_indices: Vec::new(),
    //     pre_transform: capabilities.current_transform.clone(),
    //     composite_alpha: VkCompositeAlphaFlags {
    //         opaque: true,
    //         ..VkCompositeAlphaFlags::none()
    //     },
    //     present_mode: VkPresentMode::Fifo,
    //     clipped: false,
    //     old_swapchain: None
    // }).expect("Failed to create swapchain");

    // println!("{:?}", surface_present_modes);
    // println!("{:#?}", capabilities);
    // println!("Surface supported: {}", is_surface_supported);
    // println!("Extensions supported: {:#?}", available_extensions);
    // println!("Extensions supported by device: {:#?}", device_supported_extensions);
    // println!("{:#?}", features);
    // println!("{:#?}", queue_family);
    // println!("{:#?}", properties);
    // display_properties(&properties);


    // window.start_loop();

    println!("Bye!");
}