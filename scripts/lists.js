const FUNCTIONS_TO_GENERATE = [
    'vkCreateInstance',
    'vkDestroyInstance',
    'vkEnumeratePhysicalDevices',
    'vkEnumerateInstanceExtensionProperties',
    'vkGetPhysicalDeviceFeatures',
    'vkGetPhysicalDeviceProperties',
    'vkGetPhysicalDeviceQueueFamilyProperties',
    'vkCreateDevice',
    'vkDestroyDevice',
    'vkGetDeviceQueue',
    'vkCreateBuffer',
    'vkDestroyBuffer',
    'vkGetPhysicalDeviceSurfaceSupportKHR',
    'vkDestroySurfaceKHR',
    'vkEnumerateDeviceExtensionProperties',
    'vkGetPhysicalDeviceSurfaceCapabilitiesKHR',
    'vkGetPhysicalDeviceSurfacePresentModesKHR',
    'vkCreateSwapchainKHR',
    'vkDestroySwapchainKHR'
];

const TYPES_TO_GENERATE = [
    'VkDeviceQueueCreateFlags',
    'VkExtent3D',
    'VkPhysicalDeviceFeatures',
    'VkPhysicalDeviceLimits',
    'VkPhysicalDeviceProperties',
    'VkPhysicalDeviceSparseProperties',
    'VkPhysicalDeviceType',
    'VkQueueFamilyProperties',
    'VkQueueFlags',
    'VkResult',
    'VkStructureType',
    'VkBufferCreateFlags',
    'VkBufferUsageFlags',
    'VkSharingMode',
    'VkMemoryType',
    'VkMemoryPropertyFlags',
    'VkMemoryHeap',
    'VkMemoryHeapFlags',
    'VkPhysicalDeviceMemoryProperties',
    'VkExtensionProperties',
    'VkSurfaceCapabilitiesKHR',
    'VkExtent2D',
    'VkSurfaceTransformFlagsKHR',
    'VkCompositeAlphaFlagsKHR',
    'VkImageUsageFlags',
    'VkPresentModeKHR',
    'VkFormat',
    'VkColorSpaceKHR',
    'VkSwapchainCreateFlagsKHR'
];

module.exports = {
    TYPES_TO_GENERATE,
    FUNCTIONS_TO_GENERATE
};