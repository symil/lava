// Generated by `scripts/schema.js`

use vk::VkFrom;
use std::ops::Drop;
use std::vec::Vec;
use std::ptr::null;
use vk::RawVkHandle;
use libc::c_void;

pub type RawVkInstance = RawVkHandle;

pub struct VkInstance {
    _handle: RawVkInstance,
}

impl VkInstance {
    pub fn handle(&self) -> RawVkInstance {
        self._handle
    }
} {
    pub fn new(create_info: &VkInstanceCreateInfo) -> Result<VkInstance, VkResult> {
        unsafe {
            let mut raw_create_info = RawVkInstanceCreateInfo::from(create_info);
            let raw_create_info_ptr = &mut raw_create_info as *mut RawVkInstanceCreateInfo;
            vk_call_retrieve_single(
                |ptr| vkCreateInstance(raw_create_info_ptr, null(), ptr),
                |instance| {  }
            )
        }
    }
}

impl VkFrom<RawVkInstance> for VkInstance {
    fn from(value: &RawVkInstance) -> Self {
        Self {
            _handle: *value,
        }
    }
}

impl VkFrom<VkInstance> for RawVkInstance {
    fn from(value: &VkInstance) -> Self {
        value._handle
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handle, null());
        }
    }
}

extern {
    fn vkDestroyInstance(instance: RawVkInstance, p_allocator: *const c_void);
    fn vkCreateInstance(p_create_info: *const RawVkInstanceCreateInfo, p_allocator: *const c_void, p_instance: *mut RawVkInstance)-> RawVkResult;
}