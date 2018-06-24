use std::*;
use vk::*;

pub struct VkPhysicalDevice {
    _handler: VkHandler
}

impl VkPhysicalDevice {
    pub fn from_handler(handler: VkHandler) -> VkPhysicalDevice {
        VkPhysicalDevice {
            _handler: handler
        }
    }

    pub fn get_properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let raw_properties = vk_get_physical_device_properties(self._handler);

            VkPhysicalDeviceProperties::from(&*raw_properties)
        }
    }

    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties>{
        unsafe {
            let result = vk_get_physical_device_queue_family_properties(self._handler);

            result.to_vec().into_iter().map(|raw_properties| VkQueueFamilyProperties::from(&raw_properties)).collect()
        }
    }
}

extern {
    fn vk_get_physical_device_properties(device: VkHandler) -> *const RawVkPhysicalDeviceProperties;
    fn vk_get_physical_device_queue_family_properties(device: VkHandler) -> VecInfo<RawVkQueueFamilyProperties>;
}