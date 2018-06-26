use std::default::Default;
use std::string::String;
use std::convert::From;
use std::os::raw::c_char;
use std::ffi::CString;
use std::vec::Vec;
use std::ops::Drop;
use std::ptr;
use libc::*;
use vk::*;

#[repr(C)]
pub struct RawVkDeviceCreateInfo {
    s_type: VkStructureType,
    p_next: *const void,
    flags: u32,
    queue_create_info_count: u32,
    p_queue_create_info: *mut RawVkDeviceQueueCreateInfo,
    enabled_layer_count: u32,
    pp_enabled_layer_names: *mut *mut void,
    enabled_extension_count: u32,
    pp_enabled_extension_names: *mut *mut void,
    p_enabled_features: *mut RawVkPhysicalDeviceFeatures
}

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

impl<'a> From<&'a VkDeviceCreateInfo> for RawVkDeviceCreateInfo {
    fn from(v: &'a VkDeviceCreateInfo) -> Self {
        unsafe {
            let raw_queue_create_infos = v.queue_create_infos.iter().map(|x| RawVkDeviceQueueCreateInfo::from(x)).collect();
            let raw_features = RawVkPhysicalDeviceFeatures::from(&v.enabled_features);

            RawVkDeviceCreateInfo {
                s_type: VkStructureType::DeviceCreateInfo,
                p_next: ptr::null(),
                flags: 0,
                queue_create_info_count: v.queue_create_infos.len() as u32,
                p_queue_create_info: copy_as_c_array(&raw_queue_create_infos),
                enabled_layer_count: v.enabled_layer_names.len() as u32,
                pp_enabled_layer_names: copy_as_c_string_array(&v.enabled_layer_names),
                enabled_extension_count: v.enabled_extension_names.len() as u32,
                pp_enabled_extension_names: copy_as_c_string_array(&v.enabled_extension_names),
                p_enabled_features: copy_as_c_ptr(raw_features)
            }
        }
    }
}

impl Drop for RawVkDeviceCreateInfo {
    fn drop(&mut self) {
        unsafe {
            free_c_array(self.p_queue_create_info);
            free_c_string_array(self.pp_enabled_layer_names);
            free_c_string_array(self.pp_enabled_extension_names);
            free_c_ptr(self.p_enabled_features);
        }
    }
}