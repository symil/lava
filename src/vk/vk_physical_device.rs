use std::vec::Vec;
use std::*;
use vk::*;

pub struct VkPhysicalDevice {
    pub _handler: VkHandler
}

impl VkPhysicalDevice {
    pub fn get_list(instance: &VkInstance) -> Vec<VkPhysicalDevice> {
        unsafe {
            let mut count : u32 = 0;
            let count_ptr = &mut count as *mut u32;
            let mut handler_vec : Vec<VkHandler> = Vec::new();

            vkEnumeratePhysicalDevices(instance._handler, count_ptr, ptr::null_mut());
            handler_vec.reserve(count as usize);
            handler_vec.set_len(count as usize);
            vkEnumeratePhysicalDevices(instance._handler, count_ptr, handler_vec.as_mut_ptr());

            handler_vec.into_iter().map(|handler| VkPhysicalDevice { _handler: handler }).collect()
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

    pub fn get_queue_families(&self) -> Vec<VkQueueFamilyProperties> {
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

    pub fn create_logical_device(&self, create_info: &VkDeviceCreateInfo) -> Result<VkDevice, VkResult> {
        VkDevice::new(self, create_info)
    }
}