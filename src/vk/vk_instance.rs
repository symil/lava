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
    pub fn handle(&self) -> RawVkInstance {
        self._handle
    }

    pub fn create(create_info: &VkInstanceCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let raw_instance_create_info = RawVkInstanceCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_instance_create_info as *const RawVkInstanceCreateInfo;
            vk_call_retrieve_single(|ptr| vkCreateInstance(raw_create_info_ptr, VkAllocator::null(), ptr), |x| {})
        }
    }

    pub fn get_supported_extensions(&self) -> Result<Vec<VkExtensionProperties>, VkResult> {
        unsafe {
            vk_call_retrieve_list(|count, ptr| vkEnumerateInstanceExtensionProperties(ptr::null(), count, ptr))
        }
    }

    pub fn get_physical_devices(&self) -> Result<Vec<VkPhysicalDevice>, VkResult> {
        VkPhysicalDevice::get_list(self)
    }

    pub fn create_surface_from_glfw(&self, window: &GlfwWindow) -> Result<VkSurface, VkResult> {
        VkSurface::from_glfw(self, window)
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handle, VkAllocator::null());
        }
    }
}

impl<'a> From<&'a RawVkInstance> for VkInstance {
    fn from(raw: &'a RawVkInstance) -> Self {
        Self {
            _handle: *raw
        }
    }
}