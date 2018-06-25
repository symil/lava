use std::default::Default;
use std::string::String;
use std::convert::From;
use std::os::raw::c_char;
use std::ffi::CString;
use std::ops::Drop;
use std::ptr;
use vk::*;
use utils::*;

#[repr(C)]
pub struct RawVkInstanceCreateInfo {
    s_type: VkStructureType,
    p_next: *const u8,
    flags: u32,
    p_application_info: *mut RawVkApplicationInfo,
    enabled_layer_count: u32,
    pp_enabled_layer_names: *mut *mut c_char,
    enabled_extension_count: u32,
    pp_enabled_extension_names: *mut *mut c_char,
}

#[derive(Debug)]
pub struct VkInstanceCreateInfo {
    pub application_info: VkApplicationInfo,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<String>
}

impl<'a> Default for VkInstanceCreateInfo {
    fn default() -> VkInstanceCreateInfo {
        VkInstanceCreateInfo {
            application_info: VkApplicationInfo::default(),
            enabled_layers: Vec::new(),
            enabled_extensions: Vec::new()
        }
    }
}

impl<'a> From<&'a VkInstanceCreateInfo> for RawVkInstanceCreateInfo {
    fn from(v: &'a VkInstanceCreateInfo) -> Self {
        let raw_app_info_boxed = Box::from(RawVkApplicationInfo::from(&v.application_info));

        RawVkInstanceCreateInfo {
            s_type: VkStructureType::InstanceCreateInfo,
            p_next: ptr::null(),
            flags: 0,
            p_application_info: Box::into_raw(raw_app_info_boxed),
            enabled_layer_count: v.enabled_layers.len() as u32,
            pp_enabled_layer_names: copy_as_c_string_array(&v.enabled_layers),
            enabled_extension_count: v.enabled_extensions.len() as u32,
            pp_enabled_extension_names: copy_as_c_string_array(&v.enabled_extensions)
        }
    }
}

impl Drop for RawVkInstanceCreateInfo {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.p_application_info); }
        free_c_string_array(self.pp_enabled_layer_names);
        free_c_string_array(self.pp_enabled_extension_names);
    }
}