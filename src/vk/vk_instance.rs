use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::From;
use std::ops::Drop;
use std::*;
use vk::*;
use libc::*;
use glfw::*;

pub struct VkInstance {
    _handle: RawVkInstance
}

impl VkInstance {
    pub fn create(create_info: &VkInstanceCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let raw_instance_create_info = RawVkInstanceCreateInfo::from(create_info);
            let mut handler : RawVkInstance = 0;
            let handler_ptr = &mut handler as *mut RawVkInstance;
            let create_info_ptr = &raw_instance_create_info as *const RawVkInstanceCreateInfo;

            let result = vkCreateInstance(create_info_ptr, VkAllocator::null(), handler_ptr);

            match result {
                VkResult::Success => Ok(VkInstance { _handle: handler }),
                _ => Err(result)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<VkPhysicalDevice> {
        VkPhysicalDevice::get_list(self._handle)
    }

    pub fn create_surface_from_glfw(&self, window: *mut RawGlfwWindow) -> Result<VkSurface, VkResult> {
        VkSurface::from_glfw(self._handle, window)
    }
}

impl ops::Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handle, VkAllocator::null());
        }
    }
}