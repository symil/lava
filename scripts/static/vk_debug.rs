use std::os::raw::c_char;
use std::string::String;
use std::ffi::CString;
use std::*;
use libc::*;
use vk::*;

type RawVkDebugCallback = unsafe extern fn(flags: u32, obj_type: i32, obj: u64, location: usize, code: i32, layer_prefix: *const c_char, msg: *const c_char, user_data: *mut c_void) -> RawVkBool32;
type VkDebugCallback = fn(msg: String);

#[repr(C)]
pub struct RawVkDebugReportCallbackCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    callback: RawVkDebugCallback,
    user_data: *mut c_void
}

pub struct VkDebugReportCallbackCreateInfo {
    pub flags: VkDebugReportFlagsEXT,
    pub callback: VkDebugCallback
}

unsafe extern fn c_debug_callback(flags: u32, obj_type: i32, obj: u64, location: usize, code: i32, layer_prefix: *const c_char, msg: *const c_char, user_data: *mut c_void) -> RawVkBool32 {
    let func : fn(String) = mem::transmute(user_data);
    let msg_str = copy_as_string(msg);

    func(msg_str);

    VK_FALSE
}

impl VkFrom<VkDebugReportCallbackCreateInfo> for RawVkDebugReportCallbackCreateInfo {
    fn vk_from(value: &VkDebugReportCallbackCreateInfo) -> Self {
        unsafe {
            Self {
                s_type: VkFrom::vk_from(&VkStructureType::DebugReportCallbackCreateInfoExt),
                p_next: ptr::null(),
                flags: VkFrom::vk_from(&value.flags),
                callback: c_debug_callback,
                user_data: mem::transmute::<fn(String), *mut c_void>(value.callback)
            }
        }
    }
}

type RawVkCreateDebugReportCallbackFunction = unsafe extern fn(RawVkInstance, *const RawVkDebugReportCallbackCreateInfo, *const c_void, *mut RawVkHandle) -> RawVkResult;

#[allow(non_snake_case)]
pub unsafe extern fn vkCreateDebugReportCallbackEXT(instance: RawVkInstance, create_info: *const RawVkDebugReportCallbackCreateInfo, allocator: *const c_void, handle: *mut RawVkHandle) -> RawVkResult {
    let ext_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap();
    let ext_name_c_ptr = ext_name.as_ptr();
    let func_ptr = vkGetInstanceProcAddr(instance, ext_name_c_ptr);

    if !func_ptr.is_null() {
        let func : RawVkCreateDebugReportCallbackFunction = mem::transmute(func_ptr);

        func(instance, create_info, allocator, handle)
    } else {
        VkFrom::vk_from(&VkResult::ErrorExtensionNotPresent)
    }
}

extern {
    fn vkGetInstanceProcAddr(instance: RawVkInstance, name: *const c_char) -> *const c_void;
}