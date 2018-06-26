use std::default::Default;
use std::string::String;
use std::convert::From;
use std::os::raw::c_char;
use std::ffi::CString;
use std::ops::Drop;
use std::ptr;
use libc::*;
use vk::*;

#[derive(Debug)]
pub struct VkApplicationInfo {
    pub application_name: String,
    pub application_version: [u32; 3],
    pub engine_name: String,
    pub engine_version: [u32; 3],
    pub api_version: [u32; 2]
}

#[repr(C)]
pub struct RawVkApplicationInfo {
    s_type: VkStructureType,
    p_next: *const u8,
    p_application_name: *mut c_char,
    application_version: u32,
    p_engine_name: *mut c_char,
    engine_version: u32,
    api_version: u32
}

impl Default for VkApplicationInfo {
    fn default() -> VkApplicationInfo {
        VkApplicationInfo {
            application_name: String::from("<blank>"),
            application_version: [1, 0, 0],
            engine_name: String::from("<blank>"),
            engine_version: [1, 0, 0],
            api_version: [1, 0]
        }
    }
}

impl<'a> From<&'a VkApplicationInfo> for RawVkApplicationInfo {
    fn from(v: &'a VkApplicationInfo) -> Self {
        unsafe {
            RawVkApplicationInfo {
                s_type: VkStructureType::ApplicationInfo,
                p_next: ptr::null(),
                p_application_name: copy_as_c_string(&v.application_name),
                application_version: vk_make_version(&v.application_version),
                p_engine_name: copy_as_c_string(&v.engine_name),
                engine_version: vk_make_version(&v.engine_version),
                api_version: vk_make_version(&[v.api_version[0], v.api_version[1], 0])
            }
        }
    }
}

impl Drop for RawVkApplicationInfo {
    fn drop(&mut self) {
        unsafe {
            free_c_string(self.p_application_name);
            free_c_string(self.p_engine_name);
        }
    }
}