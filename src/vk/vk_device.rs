use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::convert::From;
use std::*;
use vk::*;
use libc::*;

pub struct VkDevice {
    _handle: RawVkDevice
}

impl VkDevice {
    pub fn handle(&self) -> RawVkDevice {
        self._handle
    }

    pub fn new(physical_device: &VkPhysicalDevice, create_info: &VkDeviceCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let raw_create_info = RawVkDeviceCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkDeviceCreateInfo;
            vk_call_retrieve_single(|ptr| vkCreateDevice(physical_device.handle(), raw_create_info_ptr, VkAllocator::null(), ptr), |x| {})
        }
    }

    pub fn get_queue(&self, queue_family_index: usize, queue_index: usize) -> VkQueue {
        VkQueue::get(self, queue_family_index, queue_index)
    }

    pub fn create_buffer(&self, create_info: &VkBufferCreateInfo) -> Result<VkBuffer, VkResult> {
        VkBuffer::new(self, create_info)
    }

    pub fn create_swapchain(&self, create_info: &VkSwapchainCreateInfo) -> Result<VkSwapchain, VkResult> {
        VkSwapchain::new(self, create_info)
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self._handle, VkAllocator::null());
        }
    }
}

impl<'a> From<&'a RawVkDevice> for VkDevice {
    fn from(raw: &'a RawVkDevice) -> Self {
        Self {
            _handle: *raw
        }
    }
}