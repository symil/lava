use std::convert::From;
use std::ops::Drop;
use std::*;
use vk::*;
use libc::*;

pub struct VkSwapchain {
    _handle: RawVkSwapchain,
    _device: RawVkDevice
}

impl VkSwapchain {
    pub fn new(device: &VkDevice, create_info: &VkSwapchainCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let device_handle = device.handle();
            let raw_create_info = RawVkSwapchainCreateInfo::from(create_info);
            let raw_create_info_ptr = &raw_create_info as *const RawVkSwapchainCreateInfo;

            vk_call_retrieve_single(|ptr| vkCreateSwapchainKHR(device_handle, raw_create_info_ptr, VkAllocator::null(), ptr), |s : &mut VkSwapchain| s._device = device_handle)
        }
    }

    pub fn handle(&self) -> RawVkSwapchain {
        self._handle
    }
}

impl Drop for VkSwapchain {
    fn drop(&mut self) {
        unsafe {
            vkDestroySwapchainKHR(self._device, self._handle, VkAllocator::null());
        }
    }
}

impl<'a> From<&'a RawVkSwapchain> for VkSwapchain {
    fn from(raw: &'a RawVkSwapchain) -> Self {
        Self {
            _handle: *raw,
            _device: VK_NULL_HANDLE
        }
    }
}