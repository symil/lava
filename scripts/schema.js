const SCHEMA = {
    VkInstance: {
        new: 'vkCreateInstance',
        drop: 'vkDestroyInstance',
        getSupportedExtensions: 'vkEnumerateInstanceExtensionProperties',
        getPhysicalDevices: 'VkPhysicalDevice::getList',
        createSurfaceFromGlfw: 'VkSurface::fromGlfw',
        getLayerProperties: 'vkEnumerateInstanceLayerProperties',
        createDebugCallback: 'VkDebugReportCallback::new ; store'
    },
    VkDebugReportCallback: {
        new: 'vkCreateDebugReportCallbackEXT',
        drop: 'vkDestroyDebugReportCallbackEXT'
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
        createSwapchain: 'VkSwapchain::new',
        createImageView: 'VkImageView::new'
    },
    VkQueue: {
        get: 'vkGetDeviceQueue'
    },
    VkBuffer: {
        new: 'vkCreateBuffer',
        drop: 'vkDestroyBuffer'
    },
    VkSurface: {
        fromGlfw: 'glfwCreateWindowSurface',
        drop: 'vkDestroySurfaceKHR'
    },
    VkSwapchain: {
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