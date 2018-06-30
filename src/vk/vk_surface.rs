use std::ops::Drop;
use vk::*;
use glfw::*;

pub struct VkSurface {
    _handle: RawVkSurface,
    _instance: RawVkInstance
}

impl VkSurface {
    pub fn from_glfw(vk_instance: &VkInstance, window: &GlfwWindow) -> Result<Self, VkResult> {
        unsafe {
            vk_call_retrieve_single(
                |ptr| glfwCreateWindowSurface(vk_instance.handle(), window.handle(), VkAllocator::null(), ptr),
                |surface: &mut VkSurface| surface._instance = vk_instance.handle()
            )
        }
    }

    pub fn handle(&self) -> RawVkSurface {
        self._handle
    }
}

impl Drop for VkSurface {
    fn drop(&mut self) {
        unsafe {
            vkDestroySurfaceKHR(self._instance, self._handle, VkAllocator::null());
        }
    }
}

impl<'a> From<&'a RawVkInstance> for VkSurface {
    fn from(raw: &'a RawVkInstance) -> Self {
        Self {
            _handle: *raw,
            _instance: VK_NULL_HANDLE
        }
    }
}