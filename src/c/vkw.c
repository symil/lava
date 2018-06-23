#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <vulkan/vulkan.h>

uint32_t add_one(uint32_t x) {
    return x + 1;
}

int32_t get_kind() {
    return 1;
}

typedef struct {
    int64_t vk_result;
    void *ptr;
} RsResult;

RsResult vk_create_instance() {
    VkInstance *instance = malloc(sizeof(VkInstance));
    
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
    
    VkResult vk_result = vkCreateInstance(&create_info, NULL, instance);
    
    RsResult result = {
        .vk_result = vk_result,
        .ptr = instance
    };
    
    // printf("%zu\n", sizeof(VkInstance));
    // printf("Result from C: %lu\n", result.vk_result);

    return result;
}

void vk_destroy_instance(VkInstance *instance) {
    vkDestroyInstance(*instance, NULL);
    free(instance);
}

VkPhysicalDeviceProperties* get_first_device(VkInstance *instance) {
    uint32_t count = 0;
    vkEnumeratePhysicalDevices(*instance, &count, NULL);

    VkPhysicalDevice* devices = malloc(count * sizeof(VkPhysicalDevice));
    vkEnumeratePhysicalDevices(*instance, &count, devices);
    
    VkPhysicalDeviceProperties* first_properties = malloc(sizeof(VkPhysicalDeviceProperties));
    vkGetPhysicalDeviceProperties(devices[0], first_properties);
 
    printf("From C   : %s\n", first_properties->deviceName);

    return first_properties;
}