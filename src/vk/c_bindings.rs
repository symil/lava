use vk::*;

extern {
    pub fn vkCreateInstance(p_create_info: *const RawVkInstanceCreateInfo, p_allocator: *const VkAllocator, p_instance: *mut RawVkInstance) -> VkResult;
    pub fn vkDestroyInstance(instance: RawVkInstance, p_allocator: *const VkAllocator);
    pub fn vkEnumeratePhysicalDevices(instance: RawVkInstance, p_physical_device_count: *mut u32, p_physical_devices: *mut RawVkPhysicalDevice) -> VkResult;
    pub fn vkGetPhysicalDeviceFeatures(physical_device: RawVkPhysicalDevice, p_features: *mut RawVkPhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceProperties(physical_device: RawVkPhysicalDevice, p_properties: *mut RawVkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physical_device: RawVkPhysicalDevice, p_queue_family_property_count: *mut u32, p_queue_family_properties: *mut RawVkQueueFamilyProperties);
    pub fn vkCreateDevice(physical_device: RawVkPhysicalDevice, p_create_info: *const RawVkDeviceCreateInfo, p_allocator: *const VkAllocator, p_device: *mut RawVkDevice) -> VkResult;
    pub fn vkDestroyDevice(device: RawVkDevice, p_allocator: *const VkAllocator);
    pub fn vkEnumerateInstanceExtensionProperties(p_layer_name: *const i8, p_property_count: *mut u32, p_properties: *mut RawVkExtensionProperties) -> VkResult;
    pub fn vkEnumerateDeviceExtensionProperties(physical_device: RawVkPhysicalDevice, p_layer_name: *const i8, p_property_count: *mut u32, p_properties: *mut RawVkExtensionProperties) -> VkResult;
    pub fn vkGetDeviceQueue(device: RawVkDevice, queue_family_index: u32, queue_index: u32, p_queue: *mut RawVkQueue);
    pub fn vkCreateBuffer(device: RawVkDevice, p_create_info: *const RawVkBufferCreateInfo, p_allocator: *const VkAllocator, p_buffer: *mut RawVkBuffer) -> VkResult;
    pub fn vkDestroyBuffer(device: RawVkDevice, buffer: RawVkBuffer, p_allocator: *const VkAllocator);
    pub fn vkDestroySurfaceKHR(instance: RawVkInstance, surface: RawVkSurface, p_allocator: *const VkAllocator);
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physical_device: RawVkPhysicalDevice, queue_family_index: u32, surface: RawVkSurface, p_supported: *mut u32) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device: RawVkPhysicalDevice, surface: RawVkSurface, p_surface_capabilities: *mut RawVkSurfaceCapabilities) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device: RawVkPhysicalDevice, surface: RawVkSurface, p_present_mode_count: *mut u32, p_present_modes: *mut RawVkPresentMode) -> VkResult;
    pub fn vkCreateSwapchainKHR(device: RawVkDevice, p_create_info: *const RawVkSwapchainCreateInfo, p_allocator: *const VkAllocator, p_swapchain: *mut RawVkSwapchain) -> VkResult;
    pub fn vkDestroySwapchainKHR(device: RawVkDevice, swapchain: RawVkSwapchain, p_allocator: *const VkAllocator);
}