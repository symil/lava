use std::ops::Drop;
use vk::*;
use glfw::*;

pub struct VkSurface {
    _handle: RawVkSurfaceKHR,
    _instance: RawVkInstance
}

impl VkSurface {
    pub fn from_glfw(vk_instance: RawVkInstance, window: *mut RawGlfwWindow) -> Result<Self, VkResult> {
        unsafe {
            let mut handle : RawVkSurfaceKHR = 0;
            let ptr = &mut handle as *mut RawVkSurfaceKHR;
            let result = glfwCreateWindowSurface(vk_instance, window, VkAllocator::null(), ptr);

            match result {
                VkResult::Success => Ok(VkSurface {
                    _handle: handle,
                    _instance: vk_instance
                }),
                _ => Err(result)
            }
        }
    }
}

impl Drop for VkSurface {
    fn drop(&mut self) {
        unsafe {
            vkDestroySurfaceKHR(self._instance, self._handle, VkAllocator::null());
        }
    }
}