use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::*;
use vk::*;
use utils::*;

pub struct VkDevice {
    _handler: VkHandler
}