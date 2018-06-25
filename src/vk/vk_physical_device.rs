use std::vec::Vec;
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
            let mut raw_properties : RawVkPhysicalDeviceProperties = mem::uninitialized();
            let ptr = &mut raw_properties as *mut RawVkPhysicalDeviceProperties;

            vkGetPhysicalDeviceProperties(self._handler, ptr);

            VkPhysicalDeviceProperties::from(&raw_properties)
        }
    }

    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        unsafe {
            let mut count : u32 = 0;
            let count_ptr = &mut count as *mut u32;
            let mut queue_family_vec : Vec<RawVkQueueFamilyProperties> = Vec::new();

            vkGetPhysicalDeviceQueueFamilyProperties(self._handler, count_ptr, ptr::null_mut());
            queue_family_vec.reserve(count as usize);
            queue_family_vec.set_len(count as usize);
            vkGetPhysicalDeviceQueueFamilyProperties(self._handler, count_ptr, queue_family_vec.as_mut_ptr());

            queue_family_vec.into_iter().map(|raw_properties| VkQueueFamilyProperties::from(&raw_properties)).collect()
        }
    }

    // pub fn create_logical_device(&self) -> VkDevice {
    //     unsafe {

    //     }
    // }
}