use std::os::raw::c_char;
use std::ffi::CString;
use std::*;
use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_convert::*;
use utils::vk_ptr::*;
use vulkan::vk::*;
use vulkan::ext::*;

/// Payload of a debug report callback.
/// Contains all fields of [PFN_vkDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/PFN_vkDebugReportCallbackEXT.html).
#[derive(Debug)]
pub struct VkDebugReportCallbackMessageData {
    pub flags: VkDebugReportFlags,
    pub object_type: VkDebugReportObjectType,
    pub object: u64,
    pub location: usize,
    pub message_code: i32,
    pub layer_prefix: String,
    pub message: String
}

#[doc(hidden)]
#[repr(C)]
pub struct RawVkDebugReportCallbackCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    callback: unsafe extern fn(u32, i32, u64,  usize, i32, *const c_char, *const c_char, *mut c_void) -> u32,
    user_data: *mut c_void
}

/// Wrapper for [VkDebugReportCallbackCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html)
pub struct VkDebugReportCallbackCreateInfo {
    pub flags: VkDebugReportFlags,
    pub callback: fn(VkDebugReportCallbackMessageData)
}

unsafe extern fn c_debug_callback(flags: u32, object_type: i32, object: u64, location: usize, message_code: i32, layer_prefix: *const c_char, message: *const c_char, user_data: *mut c_void) -> u32 {
    let func : fn(VkDebugReportCallbackMessageData) = mem::transmute(user_data);

    func(VkDebugReportCallbackMessageData {
        flags: RawVkDebugReportFlags::vk_to_wrapped(&flags),
        object_type: RawVkDebugReportObjectType::vk_to_wrapped(&object_type),
        object: object,
        location: location,
        message_code: message_code,
        layer_prefix: new_string(layer_prefix),
        message: new_string(message)
    });

    0
}

impl VkWrappedType<RawVkDebugReportCallbackCreateInfo> for VkDebugReportCallbackCreateInfo {
    fn vk_to_raw(src: &VkDebugReportCallbackCreateInfo, dst: &mut RawVkDebugReportCallbackCreateInfo) {
        unsafe {
            dst.s_type = vk_to_raw_value(&VkStructureType::DebugReportCallbackCreateInfoExt);
            dst.p_next = ptr::null();
            dst.flags = vk_to_raw_value(&src.flags);
            dst.callback = c_debug_callback;
            dst.user_data = mem::transmute::<fn(VkDebugReportCallbackMessageData), *mut c_void>(src.callback);
        }
    }
}

impl VkFree for RawVkDebugReportCallbackCreateInfo {
    fn vk_free(&mut self) {
        
    }
}