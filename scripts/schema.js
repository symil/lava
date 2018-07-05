const SCHEMA = {
    VkInstance: {
        new: 'vkCreateInstance',
        drop: 'vkDestroyInstance',
        getSupportedExtensions: 'vkEnumerateInstanceExtensionProperties'
    },
    VkBuffer: {
        // new: 'vkCreateBuffer',
        // drop: 'vkDestroyBuffer'
    },
    VkPhysicalDevice: {
        // getList: 'vkEnumeratePhysicalDevices',
        // getSurfaceCapabilities: 'vkGetPhysicalDeviceSurfaceCapabilitiesKHR',
        // doesSupportSurface: 'vkGetPhysicalDeviceSurfaceSupportKHR',
        // getSurfacePresentModes: 'vkGetPhysicalDeviceSurfacePresentModesKHR',
        // createLogicalDevice: 'VkDevice::new'
    },
    VkDevice: {
        // new: 'vkCreateDevice'
    }
};

module.exports = SCHEMA;