#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <vulkan/vulkan.h>

typedef struct {
    void*   vk_handler;
    int32_t vk_result;
} VkCreateHandlerResult;

typedef struct {
    void*   ptr;
    size_t  length;
} VecInfo;

VkCreateHandlerResult vk_create_instance() {
    VkInstance instance;
    
    VkApplicationInfo app_info = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .pApplicationName = "Hello Triangle",
        .applicationVersion = VK_MAKE_VERSION(1, 0, 0),
        .pEngineName = "No Engine",
        .engineVersion = VK_MAKE_VERSION(1, 0, 0),
        .apiVersion = VK_API_VERSION_1_0
    };
    
    VkInstanceCreateInfo create_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pApplicationInfo = &app_info,
        .enabledLayerCount = 0,
        .enabledExtensionCount = 0
    };
    
    VkResult vk_result = vkCreateInstance(&create_info, NULL, &instance);
    
    VkCreateHandlerResult result = {
        .vk_result = vk_result,
        .vk_handler = instance
    };

    return result;
}

void vk_destroy_instance(VkInstance instance) {
    vkDestroyInstance(instance, NULL);
}

VecInfo vk_get_physical_device_list(VkInstance instance) {
    uint32_t count = 0;
    vkEnumeratePhysicalDevices(instance, &count, NULL);

    VkPhysicalDevice* devices = malloc(count * sizeof(VkPhysicalDevice));
    vkEnumeratePhysicalDevices(instance, &count, devices);

    VecInfo result = {
        .ptr = devices,
        .length = count
    };
    
    return result;
}

VkPhysicalDeviceProperties* vk_get_physical_device_properties(VkPhysicalDevice physical_device) {
    VkPhysicalDeviceProperties* properties = malloc(sizeof(VkPhysicalDeviceProperties));
    vkGetPhysicalDeviceProperties(physical_device, properties);

    // printf("From C   : %s\n", properties->deviceName);
    // printf("From C   : %u\n", properties->deviceID);

    return properties;
}