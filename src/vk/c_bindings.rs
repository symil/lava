use vk::*;

extern {
    pub fn vkCreateInstance(p_create_info: *const RawVkInstanceCreateInfo, p_allocator: *const VkAllocator, p_instance: *mut RawVkInstance) -> VkResult;
    pub fn vkDestroyInstance(instance: RawVkInstance, p_allocator: *const VkAllocator);
    pub fn vkEnumeratePhysicalDevices(instance: RawVkInstance, p_physical_device_count: *mut u32, p_physical_devices: *mut RawVkPhysicalDevice) -> VkResult;
    pub fn vkGetPhysicalDeviceProperties(physical_device: RawVkPhysicalDevice, p_properties: *mut RawVkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physical_device: RawVkPhysicalDevice, p_queue_family_property_count: *mut u32, p_queue_family_properties: *mut RawVkQueueFamilyProperties);
    pub fn vkCreateDevice(physical_device: RawVkPhysicalDevice, p_create_info: *const RawVkDeviceCreateInfo, p_allocator: *const VkAllocator, p_device: *mut RawVkDevice) -> VkResult;
    pub fn vkDestroyDevice(device: RawVkDevice, p_allocator: *const VkAllocator);
    pub fn vkGetDeviceQueue(device: RawVkDevice, queue_family_index: u32, queue_index: u32, p_queue: *mut RawVkQueue);
    pub fn vkCreateBuffer(device: RawVkDevice, p_create_info: *const RawVkBufferCreateInfo, p_allocator: *const VkAllocator, p_buffer: *mut RawVkBuffer) -> VkResult;
    pub fn vkDestroyBuffer(device: RawVkDevice, buffer: RawVkBuffer, p_allocator: *const VkAllocator);
}