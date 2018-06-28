use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::*;
use vk::*;
use wrapper::*;
use libc::*;

pub struct Buffer {
    _handler: VkBuffer,
    _device: VkDevice
}

impl Buffer {
    pub fn new(device: VkDevice, create_info: &VkBufferCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let mut buffer_handler : VkBuffer = 0;
            let buffer_handler_ptr = &mut buffer_handler as *mut VkBuffer;
            let raw_create_info = RawVkBufferCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkBufferCreateInfo;
            let result = vkCreateBuffer(device, raw_create_info_ptr, VkAllocator::null(), buffer_handler_ptr);

            match result {
                VkResult::Success => Ok(Buffer {
                    _handler: buffer_handler,
                    _device: device
                }),
                _ => Err(result)
            }
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            vkDestroyBuffer(self._device, self._handler, VkAllocator::null());
        }
    }
}