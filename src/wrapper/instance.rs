use std::*;
use vk::*;
use wrapper::utils::*;
use wrapper::physical_device::*;

pub struct VkInstance {
    _handler: VkHandler
}

impl VkInstance {
    pub fn create() -> Result<VkInstance, VkResult> {
        unsafe {
            let result = vk_create_instance();

            match result.code {
                VkResult::Success => Ok(VkInstance {
                    _handler: result.handler
                }),
                _ => Err(result.code)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<VkPhysicalDevice> {
        unsafe {
            let result = vk_get_physical_device_list(self._handler);
            let handler_vec = Vec::from_raw_parts(result.ptr, result.length, result.length);

            handler_vec.into_iter().map(|handler| VkPhysicalDevice::from_handler(handler)).collect()
        }
    }
}

impl ops::Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vk_destroy_instance(self._handler);
        }
    }
}

extern {
    fn vk_create_instance() -> VkCreateHandlerResult;
    fn vk_destroy_instance(instance: VkHandler);
    fn vk_get_physical_device_list(instance: VkHandler) -> VecInfo<VkHandler>;
}