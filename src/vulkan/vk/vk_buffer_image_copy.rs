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
use vulkan::vk::{VkImageSubresourceLayers,RawVkImageSubresourceLayers};
use vulkan::vk::{VkOffset3D,RawVkOffset3D};
use vulkan::vk::{VkExtent3D,RawVkExtent3D};

/// Wrapper for [VkBufferImageCopy](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferImageCopy.html).
#[derive(Debug, Clone)]
pub struct VkBufferImageCopy {
    pub buffer_offset: usize,
    pub buffer_row_length: usize,
    pub buffer_image_height: usize,
    pub image_subresource: VkImageSubresourceLayers,
    pub image_offset: VkOffset3D,
    pub image_extent: VkExtent3D,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferImageCopy {
    pub buffer_offset: u64,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: RawVkImageSubresourceLayers,
    pub image_offset: RawVkOffset3D,
    pub image_extent: RawVkExtent3D,
}

impl VkWrappedType<RawVkBufferImageCopy> for VkBufferImageCopy {
    fn vk_to_raw(src: &VkBufferImageCopy, dst: &mut RawVkBufferImageCopy) {
        dst.buffer_offset = vk_to_raw_value(&src.buffer_offset);
        dst.buffer_row_length = vk_to_raw_value(&src.buffer_row_length);
        dst.buffer_image_height = vk_to_raw_value(&src.buffer_image_height);
        dst.image_subresource = vk_to_raw_value(&src.image_subresource);
        dst.image_offset = vk_to_raw_value(&src.image_offset);
        dst.image_extent = vk_to_raw_value(&src.image_extent);
    }
}

impl VkRawType<VkBufferImageCopy> for RawVkBufferImageCopy {
    fn vk_to_wrapped(src: &RawVkBufferImageCopy) -> VkBufferImageCopy {
        VkBufferImageCopy {
            buffer_offset: u64::vk_to_wrapped(&src.buffer_offset),
            buffer_row_length: u32::vk_to_wrapped(&src.buffer_row_length),
            buffer_image_height: u32::vk_to_wrapped(&src.buffer_image_height),
            image_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.image_subresource),
            image_offset: RawVkOffset3D::vk_to_wrapped(&src.image_offset),
            image_extent: RawVkExtent3D::vk_to_wrapped(&src.image_extent),
        }
    }
}

impl Default for VkBufferImageCopy {
    fn default() -> VkBufferImageCopy {
        VkBufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: VkImageSubresourceLayers::default(),
            image_offset: VkOffset3D::default(),
            image_extent: VkExtent3D::default(),
        }
    }
}

impl VkSetup for VkBufferImageCopy {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.image_subresource, fn_table);
        VkSetup::vk_setup(&mut self.image_offset, fn_table);
        VkSetup::vk_setup(&mut self.image_extent, fn_table);
    }
}

impl VkFree for RawVkBufferImageCopy {
    fn vk_free(&mut self) {
        RawVkImageSubresourceLayers::vk_free(&mut self.image_subresource);
        RawVkOffset3D::vk_free(&mut self.image_offset);
        RawVkExtent3D::vk_free(&mut self.image_extent);
    }
}