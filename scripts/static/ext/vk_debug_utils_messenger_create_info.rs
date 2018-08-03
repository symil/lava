use std::os::raw::c_char;
use std::string::String;
use std::ffi::CString;
use std::*;
use utils::vk_type::*;
use utils::vk_convert::*;
use utils::vk_ptr::*;
use vk::vk_structure_type::*;
use vk::ext::vk_debug_utils_messenger_create_flags::*;
use vk::ext::vk_debug_utils_message_severity_flags::*;
use vk::ext::vk_debug_utils_message_type_flags::*;

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