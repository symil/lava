use std::vec::Vec;
use std::*;
use vk::*;

pub struct VkPhysicalDevice {
    _handle: RawVkPhysicalDevice
}

impl VkPhysicalDevice {
    pub fn get_list(instance: RawVkInstance) -> Vec<Self> {
        unsafe {
            let mut count : u32 = 0;
            let count_ptr = &mut count as *mut u32;
            let mut handler_vec : Vec<RawVkPhysicalDevice> = Vec::new();

            vkEnumeratePhysicalDevices(instance, count_ptr, ptr::null_mut());
            handler_vec.reserve(count as usize);
            handler_vec.set_len(count as usize);
            vkEnumeratePhysicalDevices(instance, count_ptr, handler_vec.as_mut_ptr());

            handler_vec.into_iter().map(|handler| VkPhysicalDevice { _handle: handler }).collect()
        }
    }

    pub fn does_support_surface(&self, queue_family_index: usize, surface: &VkSurface) -> Result<bool, VkResult> {
        unsafe {
            let mut supported : VkBool32 = 0;
            let result = vkGetPhysicalDeviceSurfaceSupportKHR(self._handle, queue_family_index as u32, surface.handle(), &mut supported as *mut VkBool32);

            match result {
                VkResult::Success => Ok(supported > 0),
                _ => Err(result)
            }
        }
    }

    pub fn get_supported_extensions(&self) -> Vec<VkExtensionProperties> {
        unsafe {
            let mut count : u32 = 0;
            let mut vector : Vec<RawVkExtensionProperties> = Vec::new();

            vkEnumerateDeviceExtensionProperties(self._handle, ptr::null(), &mut count as *mut u32, ptr::null_mut());
            vector.reserve(count as usize);
            vector.set_len(count as usize);
            vkEnumerateDeviceExtensionProperties(self._handle, ptr::null(), &mut count as *mut u32, vector.as_mut_ptr());

            vector.iter().map(|raw| From::from(raw)).collect()
        }
    }

    pub fn get_properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut raw_properties : RawVkPhysicalDeviceProperties = mem::uninitialized();
            let ptr = &mut raw_properties as *mut RawVkPhysicalDeviceProperties;

            vkGetPhysicalDeviceProperties(self._handle, ptr);

            VkPhysicalDeviceProperties::from(&raw_properties)
        }
    }

    pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
        unsafe {
            let mut raw_features : RawVkPhysicalDeviceFeatures = mem::uninitialized();

            vkGetPhysicalDeviceFeatures(self._handle, &mut raw_features as *mut RawVkPhysicalDeviceFeatures);

            VkPhysicalDeviceFeatures::from(&raw_features)
        }
    }

    pub fn get_queue_families(&self) -> Vec<VkQueueFamilyProperties> {
        unsafe {
            let mut count : u32 = 0;
            let count_ptr = &mut count as *mut u32;
            let mut queue_family_vec : Vec<RawVkQueueFamilyProperties> = Vec::new();

            vkGetPhysicalDeviceQueueFamilyProperties(self._handle, count_ptr, ptr::null_mut());
            queue_family_vec.reserve(count as usize);
            queue_family_vec.set_len(count as usize);
            vkGetPhysicalDeviceQueueFamilyProperties(self._handle, count_ptr, queue_family_vec.as_mut_ptr());

            queue_family_vec.iter().enumerate().map(|(index, raw_properties)| VkQueueFamilyProperties::from(raw_properties)).collect()
        }
    }

    pub fn create_logical_device(&self, create_info: &VkDeviceCreateInfo) -> Result<VkDevice, VkResult> {
        VkDevice::new(self._handle, create_info)
    }
}