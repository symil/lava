#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::ffi::*;
use std::fmt;

mod vk_types;
use vk_types::*;

mod vkw_instance;
use vkw_instance::*;

// type VkHandler = usize;
// type VkInstance = VkHandler;
// type VkPhysicalDevice = VkHandler;

#[repr(C)]
struct VecInfo<T> {
    ptr: *mut T,
    length: usize
}

// extern {
//     fn vk_create_instance() -> VkCreateHandlerResult;
//     fn vk_destroy_instance(instance: VkInstance);
//     fn vk_get_physical_device_list(instance: VkInstance) -> VecInfo<VkPhysicalDevice>;
//     fn vk_get_physical_device_properties(device: VkPhysicalDevice) -> *mut RawVkPhysicalDeviceProperties;
// }

fn display_properties(value: &VkPhysicalDeviceProperties) {
    println!("API version   : {:#?}", value.api_version);
    println!("Driver version: {:#?}", value.driver_version);
    println!("Vendor ID     : {:#?}", value.vendor_id);
    println!("Device ID     : {:#?}", value.device_id);
    println!("Device type   : {:#?}", value.device_type);
    println!("Device name   : {:#?}", value.device_name);
}

fn main() {
    // unsafe {
    //     let instance = vk_create_instance().handler;
    //     let vec_info = vk_get_physical_device_list(instance);
    //     let physical_devices = Vec::from_raw_parts(vec_info.ptr, vec_info.length, vec_info.length);

    //     let raw_properties_0 = vk_get_physical_device_properties(physical_devices[0]);
    //     let properties_0 = VkPhysicalDeviceProperties::from(&*raw_properties_0);

    //     let raw_properties_1 = vk_get_physical_device_properties(physical_devices[1]);
    //     let properties_1 = VkPhysicalDeviceProperties::from(&*raw_properties_1);
        
    //     vk_destroy_instance(instance);
    // }

    let instance = VkwInstance::create();

    println!("Bye!");
}