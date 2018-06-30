use std::convert::From;
use std::vec::Vec;
use std::*;
use vk::*;

pub struct VkPhysicalDevice {
    _handle: RawVkPhysicalDevice
}

impl VkPhysicalDevice {
    pub fn handle(&self) -> RawVkPhysicalDevice {
        self._handle
    }

    pub fn get_list(instance: &VkInstance) -> Result<Vec<Self>, VkResult> {
        unsafe {
            vk_call_retrieve_list(|count, ptr| vkEnumeratePhysicalDevices(instance.handle(), count, ptr))
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

    pub fn get_surface_capabilities(&self, surface: &VkSurface) -> Result<VkSurfaceCapabilities, VkResult> {
        unsafe {
            vk_call_retrieve_single(|ptr| vkGetPhysicalDeviceSurfaceCapabilitiesKHR(self._handle, surface.handle(), ptr), |x| {})
        }
    }

    pub fn get_surface_present_modes(&self, surface: &VkSurface) -> Result<Vec<VkPresentMode>, VkResult> {
        unsafe {
            vk_call_retrieve_list(|count, ptr| vkGetPhysicalDeviceSurfacePresentModesKHR(self._handle, surface.handle(), count, ptr))
        }
    }

    pub fn get_supported_extensions(&self) -> Result<Vec<VkExtensionProperties>, VkResult> {
        unsafe {
            vk_call_retrieve_list(|count, ptr| vkEnumerateDeviceExtensionProperties(self._handle, ptr::null(), count, ptr))
        }
    }

    pub fn get_properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            vk_call_retrieve_single_unchecked(|ptr| vkGetPhysicalDeviceProperties(self._handle, ptr), |x| {})
        }
    }

    pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
        unsafe {
            vk_call_retrieve_single_unchecked(|ptr| vkGetPhysicalDeviceFeatures(self._handle, ptr), |x| {})
        }
    }

    pub fn get_queue_families(&self) -> Vec<VkQueueFamilyProperties> {
        unsafe {
            vk_call_retrieve_list_unchecked(|count, ptr| vkGetPhysicalDeviceQueueFamilyProperties(self._handle, count, ptr))
        }
    }

    pub fn create_logical_device(&self, create_info: &VkDeviceCreateInfo) -> Result<VkDevice, VkResult> {
        VkDevice::new(self, create_info)
    }
}

impl<'a> From<&'a RawVkPhysicalDevice> for VkPhysicalDevice {
    fn from(raw: &'a RawVkPhysicalDevice) -> Self {
        Self {
            _handle: *raw
        }
    }
}