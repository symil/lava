const SCHEMA = {
    VkInstance: {
        new: 'vkCreateInstance',
        drop: 'vkDestroyInstance',
        getSupportedExtensions: 'vkEnumerateInstanceExtensionProperties',
        getPhysicalDevices: 'VkPhysicalDevice::getList'
    },
    VkPhysicalDevice: {
        getList: 'vkEnumeratePhysicalDevices',
        getSupportedExtensions: 'vkEnumerateDeviceExtensionProperties',
        getProperties: 'vkGetPhysicalDeviceProperties',
        getFeatures: 'vkGetPhysicalDeviceFeatures',
        getQueueFamilies: 'vkGetPhysicalDeviceQueueFamilyProperties',
        createLogicalDevice: 'VkDevice::new'
    },
    VkDevice: {
        new: 'vkCreateDevice',
        drop: 'vkDestroyDevice',
        getQueue: 'VkQueue::get',
        createBuffer: 'VkBuffer::new',
        createSwapchain: 'VkSwapchainKHR::new'
    },
    VkQueue: {
        get: 'vkGetDeviceQueue'
    },
    VkBuffer: {
        new: 'vkCreateBuffer',
        drop: 'vkDestroyBuffer'
    },
    VkSwapchainKHR: {
        new: 'vkCreateSwapchainKHR',
        drop: 'vkDestroySwapchainKHR'
    }
};

module.exports = SCHEMA;