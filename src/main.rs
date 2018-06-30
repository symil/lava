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

fn display_properties(value: &VkPhysicalDeviceProperties) {
    println!("API version   : {:#?}", value.api_version);
    println!("Driver version: {:#?}", value.driver_version);
    println!("Vendor ID     : {:#?}", value.vendor_id);
    println!("Device ID     : {:#?}", value.device_id);
    println!("Device type   : {:#?}", value.device_type);
    println!("Device name   : {:#?}", value.device_name);
}

fn main() {
    let glfw = GlfwInstance::new();
    let required_extensions = glfw.get_required_vulkan_extensions().unwrap();
    let window = glfw.create_window(800, 600, "Vulkan");
    let instance = VkInstance::create(&VkInstanceCreateInfo {
        application_info: Default::default(),
        enabled_layers: Vec::new(),
        enabled_extensions: required_extensions
    }).expect("Failed to create VkInstance");

    let surface = instance.create_surface_from_glfw(window.as_raw()).expect("Failed to create VkSurface");

    let physical_devices = instance.get_physical_devices();

    let physical_device = &physical_devices[0];
    
    let properties = physical_device.get_properties();
    let queue_families = physical_device.get_queue_families();
    let queue_family = &queue_families[0];
    let device = physical_device.create_logical_device(&Default::default()).expect("Unable to initialize Device");
    let queue = device.get_queue(0, 0);
    let buffer = device.create_buffer(&VkBufferCreateInfo {
        size: 128,
        flags: VkBufferCreateFlags::none(),
        usage: VkBufferUsageFlags::all(),
        sharing_mode: VkSharingMode::Exclusive,
        queue_families: Vec::new()
    }).expect("Unable to create Buffer");

    println!("{:#?}", queue_family);
    display_properties(&properties);


    window.start_loop();

    println!("Bye!");
}