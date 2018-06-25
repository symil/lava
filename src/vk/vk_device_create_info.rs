use std::default::Default;
use std::string::String;
use std::convert::From;
use std::os::raw::c_char;
use std::ffi::CString;
use std::vec::Vec;
use std::ops::Drop;
use std::ptr;
use vk::*;

pub struct VkDeviceCreateInfo {
    pub queue_create_infos: Vec<VkDeviceQueueCreateInfo>,
    pub enabled_layer_names: Vec<String>,
    pub enabled_extension_names: Vec<String>,
    pub enabled_features: VkPhysicalDeviceFeatures
}

impl Default for VkDeviceCreateInfo {
    fn default() -> Self {
        VkDeviceCreateInfo {
            queue_create_infos: vec![Default::default()],
            enabled_layer_names: Vec::new(),
            enabled_extension_names: Vec::new(),
            enabled_features: Default::default()
        }
    }
}