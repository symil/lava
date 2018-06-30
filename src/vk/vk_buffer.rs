use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::*;
use vk::*;
use libc::*;

pub struct VkBuffer {
    _handler: RawVkBuffer,
    _device: RawVkDevice
}

impl VkBuffer {
    pub fn new(device: RawVkDevice, create_info: &VkBufferCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let mut buffer_handler : RawVkBuffer = 0;
            let buffer_handler_ptr = &mut buffer_handler as *mut RawVkBuffer;
            let raw_create_info = RawVkBufferCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkBufferCreateInfo;
            let result = vkCreateBuffer(device, raw_create_info_ptr, VkAllocator::null(), buffer_handler_ptr);

            match result {
                VkResult::Success => Ok(VkBuffer {
                    _handler: buffer_handler,
                    _device: device
                }),
                _ => Err(result)
            }
        }
    }
}

impl Drop for VkBuffer {
    fn drop(&mut self) {
        unsafe {
            vkDestroyBuffer(self._device, self._handler, VkAllocator::null());
        }
    }
}