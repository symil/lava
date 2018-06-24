use std::*;
use vk::*;
use wrapper::utils::*;

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
}

extern {
    fn vk_get_physical_device_properties(device: VkHandler) -> *const RawVkPhysicalDeviceProperties;
}