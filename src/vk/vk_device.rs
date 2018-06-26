use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ops::Drop;
use std::*;
use vk::*;
use libc::*;

pub struct VkDevice {
    pub _handler: VkHandler
}

impl VkDevice {
    pub fn new(physical_device: &VkPhysicalDevice, create_info: &VkDeviceCreateInfo) -> Result<VkDevice, VkResult> {
        unsafe {
            let mut device_handler : VkHandler = 0;
            let device_handler_ptr = &mut device_handler as *mut VkHandler;
            let raw_create_info = RawVkDeviceCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkDeviceCreateInfo;
            let result = vkCreateDevice(physical_device._handler, raw_create_info_ptr, VkAllocator::null(), device_handler_ptr);

            match result {
                VkResult::Success => Ok(VkDevice { _handler: device_handler }),
                _ => Err(result)
            }
        }
    }

    pub fn get_queue(&self, queue_family_index: usize, queue_index: usize) -> VkQueue {
        VkQueue::from_device(self, queue_family_index, queue_index)
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self._handler, VkAllocator::null());
        }
    }
}