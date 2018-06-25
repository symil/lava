use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::From;
use std::ops::Drop;
use std::*;
use vk::*;
use utils::*;

pub struct VkInstance {
    _handler: VkHandler
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
                VkResult::Success => Ok(VkInstance {
                    _handler: handler
                }),
                _ => Err(result)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<VkPhysicalDevice> {
        unsafe {
            let mut count : u32 = 0;
            let count_ptr = &mut count as *mut u32;
            let mut handler_vec : Vec<VkHandler> = Vec::new();

            vkEnumeratePhysicalDevices(self._handler, count_ptr, ptr::null_mut());
            handler_vec.reserve(count as usize);
            handler_vec.set_len(count as usize);
            vkEnumeratePhysicalDevices(self._handler, count_ptr, handler_vec.as_mut_ptr());

            handler_vec.into_iter().map(|handler| VkPhysicalDevice::from_handler(handler)).collect()
        }
    }
}

impl ops::Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handler, VkAllocator::null());
        }
    }
}