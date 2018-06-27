use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::*;
use vk::*;
use libc::*;

pub struct VkBuffer {
    pub _handler: VkHandler,
    _device_handler: VkHandler
}

impl VkBuffer {
    pub fn new(device: &VkDevice, create_info: &VkBufferCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let mut buffer_handler : VkHandler = 0;
            let buffer_handler_ptr = &mut buffer_handler as *mut VkHandler;
            let raw_create_info = RawVkBufferCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkBufferCreateInfo;
            let result = vkCreateBuffer(device._handler, raw_create_info_ptr, VkAllocator::null(), buffer_handler_ptr);

            match result {
                VkResult::Success => Ok(VkBuffer {
                    _handler: buffer_handler,
                    _device_handler: device._handler
                }),
                _ => Err(result)
            }
        }
    }
}

impl Drop for VkBuffer {
    fn drop(&mut self) {
        unsafe {
            vkDestroyBuffer(self._device_handler, self._handler, VkAllocator::null());
        }
    }
}