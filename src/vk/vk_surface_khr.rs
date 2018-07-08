// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;
use glfw::*;

pub type RawVkSurfaceKHR = RawVkHandle;

#[derive(Debug)]
pub struct VkSurfaceKHR {
    _handle: RawVkSurfaceKHR,
    _instance: RawVkInstance,
}

impl VkSurfaceKHR {
    
    pub fn handle(&self) -> RawVkSurfaceKHR {
        self._handle
    }
    
    pub fn from_glfw(instance: &VkInstance, glfw_window: &GlfwWindow) -> Result<VkSurfaceKHR, VkResult> {
        unsafe {
            let instance_handle = instance.handle();
            vk_call_retrieve_single(
                |ptr| glfwCreateWindowSurface(instance_handle, glfw_window.handle(), null(), ptr),
                |surface_khr : &mut VkSurfaceKHR| { surface_khr._instance = instance_handle; }
            )
        }
    }
}

impl VkFrom<VkSurfaceKHR> for RawVkSurfaceKHR {
    
    fn vk_from(value: &VkSurfaceKHR) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkSurfaceKHR> for VkSurfaceKHR {
    
    fn vk_from(value: &RawVkSurfaceKHR) -> Self {
        Self {
            _handle: *value,
            _instance: VK_NULL_HANDLE,
        }
    }
}

impl Drop for VkSurfaceKHR {
    
    fn drop(&mut self) {
        unsafe {
            vkDestroySurfaceKHR(self._instance, self._handle, null());
        }
    }
}

extern {
    fn vkDestroySurfaceKHR(instance: RawVkInstance, surface: RawVkSurfaceKHR, p_allocator: *const c_void);
    fn glfwCreateWindowSurface(instance: RawVkInstance, window: *mut RawGlfwWindow, allocator: *const c_void, surface: *mut RawVkSurfaceKHR)-> RawVkResult;
}