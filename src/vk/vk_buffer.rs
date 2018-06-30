use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::convert::From;
use std::*;
use vk::*;
use libc::*;

pub struct VkBuffer {
    _handle: RawVkBuffer,
    _device: RawVkDevice
}

impl VkBuffer {
    pub fn new(device: &VkDevice, create_info: &VkBufferCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let raw_create_info = RawVkBufferCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkBufferCreateInfo;

            vk_call_retrieve_single(
                |ptr| vkCreateBuffer(device.handle(), raw_create_info_ptr, VkAllocator::null(), ptr),
                |buffer: &mut VkBuffer| buffer._device = device.handle()
            )
        }
    }
}

impl Drop for VkBuffer {
    fn drop(&mut self) {
        unsafe {
            vkDestroyBuffer(self._device, self._handle, VkAllocator::null());
        }
    }
}

impl<'a> From<&'a RawVkBuffer> for VkBuffer {
    fn from(raw: &'a RawVkBuffer) -> Self {
        Self {
            _handle: *raw,
            _device: VK_NULL_HANDLE
        }
    }
}