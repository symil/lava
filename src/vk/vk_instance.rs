use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::From;
use std::ops::Drop;
use std::*;
use vk::*;
use libc::*;

pub struct VkInstance {
    pub _handler: VkHandler
}

impl VkInstance {
    pub fn create(create_info: &VkInstanceCreateInfo) -> Result<VkInstance, VkResult> {
        unsafe {
            let raw_instance_create_info = RawVkInstanceCreateInfo::from(create_info);
            let mut handler : VkHandler = 0;
            let handler_ptr = &mut handler as *mut VkHandler;
            let create_info_ptr = &raw_instance_create_info as *const RawVkInstanceCreateInfo;

            let result = vkCreateInstance(create_info_ptr, VkAllocator::null(), handler_ptr);

            match result {
                VkResult::Success => Ok(VkInstance { _handler: handler }),
                _ => Err(result)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<VkPhysicalDevice> {
        VkPhysicalDevice::get_list(self)
    }
}

impl ops::Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handler, VkAllocator::null());
        }
    }
}