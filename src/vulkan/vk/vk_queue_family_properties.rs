// Generated by `scripts/generate.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vulkan::vk::*;
use vulkan::vk::{VkQueueFlags,RawVkQueueFlags};
use vulkan::vk::{VkExtent3D,RawVkExtent3D};

/// Wrapper for [VkQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties.html).
#[derive(Debug, Clone)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: VkQueueFlags,
    pub queue_count: usize,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: VkExtent3D,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkQueueFamilyProperties {
    pub queue_flags: RawVkQueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: RawVkExtent3D,
}

impl VkWrappedType<RawVkQueueFamilyProperties> for VkQueueFamilyProperties {
    fn vk_to_raw(src: &VkQueueFamilyProperties, dst: &mut RawVkQueueFamilyProperties) {
        dst.queue_flags = vk_to_raw_value(&src.queue_flags);
        dst.queue_count = vk_to_raw_value(&src.queue_count);
        dst.timestamp_valid_bits = src.timestamp_valid_bits;
        dst.min_image_transfer_granularity = vk_to_raw_value(&src.min_image_transfer_granularity);
    }
}

impl VkRawType<VkQueueFamilyProperties> for RawVkQueueFamilyProperties {
    fn vk_to_wrapped(src: &RawVkQueueFamilyProperties) -> VkQueueFamilyProperties {
        VkQueueFamilyProperties {
            queue_flags: RawVkQueueFlags::vk_to_wrapped(&src.queue_flags),
            queue_count: u32::vk_to_wrapped(&src.queue_count),
            timestamp_valid_bits: src.timestamp_valid_bits,
            min_image_transfer_granularity: RawVkExtent3D::vk_to_wrapped(&src.min_image_transfer_granularity),
        }
    }
}

impl Default for VkQueueFamilyProperties {
    fn default() -> VkQueueFamilyProperties {
        VkQueueFamilyProperties {
            queue_flags: Default::default(),
            queue_count: 0,
            timestamp_valid_bits: 0,
            min_image_transfer_granularity: Default::default(),
        }
    }
}

impl VkSetup for VkQueueFamilyProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.min_image_transfer_granularity, fn_table);
    }
}

impl VkFree for RawVkQueueFamilyProperties {
    fn vk_free(&self) {
        
    }
}