use vk::*;

extern {
    pub fn vkCreateInstance(create_info: *const RawVkInstanceCreateInfo, allocator: *const VkAllocator, instance: *mut VkHandler) -> VkResult;
    pub fn vkDestroyInstance(instance: VkHandler, allocator: *const VkAllocator);
    pub fn vkEnumeratePhysicalDevices(instance: VkHandler, count: *mut u32, ptr: *mut VkHandler);
    pub fn vkGetPhysicalDeviceProperties(physical_device: VkHandler, ptr: *mut RawVkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physical_device: VkHandler, count: *mut u32, ptr: *mut RawVkQueueFamilyProperties);
    pub fn vkCreateDevice(physical_device: VkHandler, create_info: *const RawVkDeviceCreateInfo, allocator: *const VkAllocator, device: *mut VkHandler) -> VkResult;
    pub fn vkDestroyDevice(device: VkHandler, allocator: *const VkAllocator);
    pub fn vkGetDeviceQueue(device: VkHandler, queue_family_index: u32, queue_index: u32, queue: *mut VkHandler);
}