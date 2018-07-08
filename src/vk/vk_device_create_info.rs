// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr::null;
use libc::*;

#[repr(C)]
pub struct RawVkDeviceCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkDeviceCreateFlags,
    queue_create_info_count: u32,
    queue_create_infos: *mut RawVkDeviceQueueCreateInfo,
    enabled_layer_count: u32,
    enabled_layer_names: *mut *mut c_char,
    enabled_extension_count: u32,
    enabled_extension_names: *mut *mut c_char,
    enabled_features: *mut RawVkPhysicalDeviceFeatures,
}

#[derive(Debug)]
pub struct VkDeviceCreateInfo {
    pub flags: VkDeviceCreateFlags,
    pub queue_create_infos: Vec<VkDeviceQueueCreateInfo>,
    pub enabled_layer_names: Vec<String>,
    pub enabled_extension_names: Vec<String>,
    pub enabled_features: VkPhysicalDeviceFeatures,
}

impl VkFrom<VkDeviceCreateInfo> for RawVkDeviceCreateInfo {
    
    fn vk_from(value: &VkDeviceCreateInfo) -> Self {
        unsafe {
            Self {
                s_type: VkFrom::vk_from(&VkStructureType::DeviceCreateInfo),
                next: null(),
                flags: VkFrom::vk_from(&value.flags),
                queue_create_info_count: value.queue_create_infos.len() as u32,
                queue_create_infos: copy_as_c_array(&value.queue_create_infos.iter().map(|x| VkFrom::vk_from(x)).collect()),
                enabled_layer_count: value.enabled_layer_names.len() as u32,
                enabled_layer_names: copy_as_c_string_array(&value.enabled_layer_names),
                enabled_extension_count: value.enabled_extension_names.len() as u32,
                enabled_extension_names: copy_as_c_string_array(&value.enabled_extension_names),
                enabled_features: copy_as_c_ptr(VkFrom::vk_from(&value.enabled_features)),
            }
        }
    }
}

impl Drop for RawVkDeviceCreateInfo {
    
    fn drop(&mut self) {
        unsafe {
            free_c_array(self.queue_create_infos);
            free_c_string_array(self.enabled_layer_names);
            free_c_string_array(self.enabled_extension_names);
            free_c_ptr(self.enabled_features);
        }
    }
}