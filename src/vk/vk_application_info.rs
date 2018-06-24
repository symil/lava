use std::default::Default;
use std::string::String;
use std::convert::From;
use std::os::raw::c_char;
use std::ffi::CString;
use std::ptr;
use vk::*;

#[derive(Debug)]
pub struct VkApplicationInfo<'a> {
    pub application_name: &'a str,
    pub application_version: [u32; 3],
    pub engine_name: &'a str,
    pub engine_version: [u32; 3],
    pub api_version: [u32; 2]
}

impl<'a> Default for VkApplicationInfo<'a> {
    fn default() -> VkApplicationInfo<'a> {
        VkApplicationInfo {
            application_name: "<blank>",
            application_version: [1, 0, 0],
            engine_name: "<blank>",
            engine_version: [1, 0, 0],
            api_version: [1, 0]
        }
    }
}