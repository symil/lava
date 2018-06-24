use std::convert::From;
use vk::vk_extent_3d::*;
use vk::vk_queue_capabilities::*;

#[repr(C)]
pub struct RawVkQueueFamilyProperties {
    queue_flags: u32,
    queue_count: u32,
    timestamp_valid_bits: u32,
    min_image_transfer_granularity: RawVkExtent3D
}

#[derive(Debug)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: VkQueueCapabilities,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: VkExtent3D
}

impl<'a> From<&'a RawVkQueueFamilyProperties> for VkQueueFamilyProperties {
    fn from(value: &'a RawVkQueueFamilyProperties) -> Self {
        VkQueueFamilyProperties {
            queue_flags: VkQueueCapabilities::from(value.queue_flags),
            queue_count: value.queue_count,
            timestamp_valid_bits: value.timestamp_valid_bits,
            min_image_transfer_granularity: VkExtent3D::from(&value.min_image_transfer_granularity)
        }
    }
}