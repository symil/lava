use std::os::raw::c_char;
use std::ffi::CString;
use std::*;
use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_convert::*;
use utils::vk_ptr::*;
use vulkan::vk::*;
use vulkan::ext::*;

type VkDebugUtilsMessengerCallback = fn(VkDebugUtilsMessageSeverityFlags, VkDebugUtilsMessageTypeFlags, VkDebugUtilsMessengerCallbackData);

#[doc(hidden)]
#[repr(C)]
pub struct RawVkDebugUtilsMessengerCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    message_severity: u32,
    message_type: u32,
    user_callback: unsafe extern fn(RawVkDebugUtilsMessageSeverityFlags, RawVkDebugUtilsMessageTypeFlags, *const RawVkDebugUtilsMessengerCallbackData, *mut c_void) -> u32,
    user_data: *mut c_void
}

/// Wrapper for [VkDebugUtilsMessengerCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDebugUtilsMessengerCreateInfo.html).
/// 
/// NOT IMPLEMENTED YET
pub struct VkDebugUtilsMessengerCreateInfo {
    pub flags: VkDebugUtilsMessengerCreateFlags,
    pub message_severity: VkDebugUtilsMessageSeverityFlags,
    pub message_type: VkDebugUtilsMessageTypeFlags,
    pub user_callback: VkDebugUtilsMessengerCallback
}

unsafe extern fn raw_callback(message_severity: RawVkDebugUtilsMessageSeverityFlags, message_types: RawVkDebugUtilsMessageTypeFlags, callback_data: *const RawVkDebugUtilsMessengerCallbackData, user_data: *mut c_void) -> u32 {
    let func : VkDebugUtilsMessengerCallback = mem::transmute(user_data);

    func(
        RawVkDebugUtilsMessageSeverityFlags::vk_to_wrapped(&message_severity),
        RawVkDebugUtilsMessageSeverityFlags::vk_to_wrapped(&message_types),
        RawVkDebugUtilsMessengerCallbackData::vk_to_wrapped(&*callback_data)
    );

    0
}

impl VkWrappedType<RawVkDebugUtilsMessengerCreateInfo> for VkDebugUtilsMessengerCreateInfo {
    fn vk_to_raw(src: &VkDebugUtilsMessengerCreateInfo, dst: &mut RawVkDebugUtilsMessengerCreateInfo) {
        unsafe {
            dst.s_type = vk_to_raw_value(&VkStructureType::DebugUtilsMessengerCreateInfoExt);
            dst.p_next = ptr::null();
            dst.flags = vk_to_raw_value(&src.flags);
            dst.message_severity = vk_to_raw_value(&src.message_severity);
            dst.message_type = vk_to_raw_value(&src.message_type);
            dst.user_callback = raw_callback;
            dst.user_data = mem::transmute::<VkDebugUtilsMessengerCallback, *mut c_void>(src.user_callback);
        }
    }
}

impl VkFree for RawVkDebugUtilsMessengerCreateInfo {
    fn vk_free(&self) {
        
    }
}