const SCHEMA = {
    VkInstance: {
        new: 'vkCreateInstance',
        drop: 'vkDestroyInstance',
        getSupportedExtensions: 'vkEnumerateInstanceExtensionProperties',
        getPhysicalDevices: 'VkPhysicalDevice::getList',
        createSurfaceFromGlfw: 'VkSurfaceKHR::fromGlfw',
        getLayerProperties: 'vkEnumerateInstanceLayerProperties',
        createDebugCallback: 'vkCreateDebugReportCallbackEXT'
    },
    VkPhysicalDevice: {
        getList: 'vkEnumeratePhysicalDevices',
        getSupportedExtensions: 'vkEnumerateDeviceExtensionProperties',
        getProperties: 'vkGetPhysicalDeviceProperties',
        getFeatures: 'vkGetPhysicalDeviceFeatures',
        getQueueFamilies: 'vkGetPhysicalDeviceQueueFamilyProperties',
        createLogicalDevice: 'VkDevice::new',
        doesSupportSurface: 'vkGetPhysicalDeviceSurfaceSupportKHR',
        getSurfaceCapabilities: 'vkGetPhysicalDeviceSurfaceCapabilitiesKHR',
        getSurfacePresentModes: 'vkGetPhysicalDeviceSurfacePresentModesKHR',
        getSurfaceFormats: 'vkGetPhysicalDeviceSurfaceFormatsKHR'
    },
    VkDevice: {
        new: 'vkCreateDevice',
        drop: 'vkDestroyDevice',
        getQueue: 'VkQueue::get',
        createBuffer: 'VkBuffer::new',
        createSwapchain: 'VkSwapchainKHR::new',
        createImageView: 'VkImageView::new'
    },
    VkQueue: {
        get: 'vkGetDeviceQueue'
    },
    VkBuffer: {
        new: 'vkCreateBuffer',
        drop: 'vkDestroyBuffer'
    },
    VkSurfaceKHR: {
        fromGlfw: 'glfwCreateWindowSurface',
        drop: 'vkDestroySurfaceKHR'
    },
    VkSwapchainKHR: {
        new: 'vkCreateSwapchainKHR',
        drop: 'vkDestroySwapchainKHR',
        getImages: 'vkGetSwapchainImagesKHR'
    },
    VkImageView: {
        new: 'vkCreateImageView',
        drop: 'vkDestroyImageView'
    }
};

module.exports = SCHEMA;