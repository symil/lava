use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
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

    pub fn new(physical_device: RawVkPhysicalDevice, create_info: &VkDeviceCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let mut device_handler : RawVkDevice = 0;
            let device_handler_ptr = &mut device_handler as *mut RawVkDevice;
            let raw_create_info = RawVkDeviceCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkDeviceCreateInfo;
            let result = vkCreateDevice(physical_device, raw_create_info_ptr, VkAllocator::null(), device_handler_ptr);

            match result {
                VkResult::Success => Ok(VkDevice { _handle: device_handler }),
                _ => Err(result)
            }
        }
    }

    pub fn get_queue(&self, queue_family_index: usize, queue_index: usize) -> VkQueue {
        VkQueue::get(self._handle, queue_family_index, queue_index)
    }

    pub fn create_buffer(&self, create_info: &VkBufferCreateInfo) -> Result<VkBuffer, VkResult> {
        VkBuffer::new(self._handle, create_info)
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