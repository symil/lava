use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::From;
use std::ops::Drop;
use std::*;
use vk::*;
use wrapper::*;
use libc::*;

pub struct Instance {
    _handler: VkInstance
}

impl Instance {
    pub fn create(create_info: &VkInstanceCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let raw_instance_create_info = RawVkInstanceCreateInfo::from(create_info);
            let mut handler : VkInstance = 0;
            let handler_ptr = &mut handler as *mut VkHandle;
            let create_info_ptr = &raw_instance_create_info as *const RawVkInstanceCreateInfo;

            let result = vkCreateInstance(create_info_ptr, VkAllocator::null(), handler_ptr);

            match result {
                VkResult::Success => Ok(Instance { _handler: handler }),
                _ => Err(result)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<PhysicalDevice> {
        PhysicalDevice::get_list(self._handler)
    }
}

impl ops::Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handler, VkAllocator::null());
        }
    }
}