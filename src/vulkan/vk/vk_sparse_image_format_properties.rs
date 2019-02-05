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
use vulkan::vk::{VkImageAspectFlags,RawVkImageAspectFlags};
use vulkan::vk::{VkExtent3D,RawVkExtent3D};
use vulkan::vk::{VkSparseImageFormatFlags,RawVkSparseImageFormatFlags};

/// Wrapper for [VkSparseImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSparseImageFormatProperties.html).
#[derive(Debug, Clone)]
pub struct VkSparseImageFormatProperties {
    pub aspect_mask: VkImageAspectFlags,
    pub image_granularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageFormatProperties {
    pub aspect_mask: RawVkImageAspectFlags,
    pub image_granularity: RawVkExtent3D,
    pub flags: RawVkSparseImageFormatFlags,
}

impl VkWrappedType<RawVkSparseImageFormatProperties> for VkSparseImageFormatProperties {
    fn vk_to_raw(src: &VkSparseImageFormatProperties, dst: &mut RawVkSparseImageFormatProperties) {
        dst.aspect_mask = vk_to_raw_value(&src.aspect_mask);
        dst.image_granularity = vk_to_raw_value(&src.image_granularity);
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl VkRawType<VkSparseImageFormatProperties> for RawVkSparseImageFormatProperties {
    fn vk_to_wrapped(src: &RawVkSparseImageFormatProperties) -> VkSparseImageFormatProperties {
        VkSparseImageFormatProperties {
            aspect_mask: RawVkImageAspectFlags::vk_to_wrapped(&src.aspect_mask),
            image_granularity: RawVkExtent3D::vk_to_wrapped(&src.image_granularity),
            flags: RawVkSparseImageFormatFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl Default for VkSparseImageFormatProperties {
    fn default() -> VkSparseImageFormatProperties {
        VkSparseImageFormatProperties {
            aspect_mask: VkImageAspectFlags::default(),
            image_granularity: VkExtent3D::default(),
            flags: VkSparseImageFormatFlags::default(),
        }
    }
}

impl VkSetup for VkSparseImageFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.image_granularity, fn_table);
    }
}

impl VkFree for RawVkSparseImageFormatProperties {
    fn vk_free(&mut self) {
        RawVkExtent3D::vk_free(&mut self.image_granularity);
    }
}