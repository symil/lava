use std::vec::Vec;
use std::default::Default;
use vk::*;

pub struct VkDeviceQueueCreateInfo {
    pub flags: VkDeviceQueueCreateFlags,
    pub queue_family_index: usize,
    pub queue_count: u32,
    pub queue_priorities: Vec<f32>
}

impl Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        VkDeviceQueueCreateInfo {
            flags: VkDeviceQueueCreateFlags {
                protected: false
            },
            queue_family_index: 0,
            queue_count: 1,
            queue_priorities: vec![1.0]
        }
    }
}