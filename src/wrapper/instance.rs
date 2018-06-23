use std::*;
use parent::vk_types::*;
use common::*;

pub struct VkwInstance {
    _handler: VkHandler
}

impl VkwInstance {
    pub fn create() -> Result<VkwInstance, VkResult> {
        unsafe {
            let result = vk_create_instance();

            match result.code {
                VkResult::Success => Ok(VkwInstance {
                    _handler: result.handler
                }),
                _ => Err(result.code)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<PhysicalDevice> {
        unsafe {
            let result = vk_get_physical_device_list(self._handler);

            Vec::from_raw_parts(result.ptr, result.length, result.length)
        }
    }
}

impl ops::Drop for VkwInstance {
    fn drop(&mut self) {
        unsafe {
            vk_destroy_instance(self._handler);
        }
    }
}

extern {
    fn vk_create_instance() -> VkCreateHandlerResult;
    fn vk_destroy_instance(instance: VkHandler);
    fn vk_get_physical_device_list(instance: VkInstance) -> VecInfo<VkPhysicalDevice>;
}