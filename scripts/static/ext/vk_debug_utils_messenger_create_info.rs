use std::os::raw::c_char;
use std::ffi::CString;
use std::*;
use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_convert::*;
use utils::vk_ptr::*;
use vulkan::vk::*;
use vulkan::ext::*;

#[doc(hidden)]
#[repr(C)]
pub struct RawVkDebugUtilsMessengerCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    message_severity: u32,
    message_type: u32,
    user_callback: extern fn(),
    user_data: *mut c_void
}

/// Wrapper for [VkDebugUtilsMessengerCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDebugUtilsMessengerCreateInfo.html)
pub struct VkDebugUtilsMessengerCreateInfo {
    pub flags: VkDebugUtilsMessengerCreateFlags,
    pub message_severity: VkDebugUtilsMessageSeverityFlags,
    pub message_type: VkDebugUtilsMessageTypeFlags,
    pub user_callback: extern fn(),
    pub user_data: *mut c_void
}

impl VkWrappedType<RawVkDebugUtilsMessengerCreateInfo> for VkDebugUtilsMessengerCreateInfo {
    fn vk_to_raw(src: &VkDebugUtilsMessengerCreateInfo, dst: &mut RawVkDebugUtilsMessengerCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DebugUtilsMessengerCreateInfoExt);
        dst.p_next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.message_severity = vk_to_raw_value(&src.message_severity);
        dst.message_type = vk_to_raw_value(&src.message_type);
        dst.user_callback = src.user_callback;
        dst.user_data = src.user_data;
    }
}

impl VkFree for RawVkDebugUtilsMessengerCreateInfo {
    fn vk_free(&mut self) {
        
    }
}