#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::ffi::*;
use std::fmt;

mod vk;
use vk::*;

mod utils;

fn display_properties(value: &VkPhysicalDeviceProperties) {
    println!("API version   : {:#?}", value.api_version);
    println!("Driver version: {:#?}", value.driver_version);
    println!("Vendor ID     : {:#?}", value.vendor_id);
    println!("Device ID     : {:#?}", value.device_id);
    println!("Device type   : {:#?}", value.device_type);
    println!("Device name   : {:#?}", value.device_name);
}

fn main() {
    let instance = VkInstance::create(&VkInstanceCreateInfo::default()).unwrap();
    let physical_devices = instance.get_physical_devices();
    
    let properties = physical_devices[0].get_properties();

    display_properties(&properties);

    println!("Bye!");
}