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
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::vk::{VkImageCreateFlags,RawVkImageCreateFlags};
use vulkan::vk::{VkImageType,RawVkImageType};
use vulkan::vk::{VkFormat,RawVkFormat};
use vulkan::vk::{VkExtent3D,RawVkExtent3D};
use vulkan::vk::{VkSampleCountFlags,RawVkSampleCountFlags};
use vulkan::vk::{VkImageTiling,RawVkImageTiling};
use vulkan::vk::{VkImageUsageFlags,RawVkImageUsageFlags};
use vulkan::vk::{VkSharingMode,RawVkSharingMode};
use vulkan::vk::{VkImageLayout,RawVkImageLayout};

/// Wrapper for [VkImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkImageCreateInfo<'a> {
    pub flags: VkImageCreateFlags,
    pub image_type: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mip_levels: usize,
    pub array_layers: usize,
    pub samples: VkSampleCountFlags,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_family_indices: &'a [usize],
    pub initial_layout: VkImageLayout,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkImageCreateFlags,
    pub image_type: RawVkImageType,
    pub format: RawVkFormat,
    pub extent: RawVkExtent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: RawVkSampleCountFlags,
    pub tiling: RawVkImageTiling,
    pub usage: RawVkImageUsageFlags,
    pub sharing_mode: RawVkSharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *mut u32,
    pub initial_layout: RawVkImageLayout,
}

impl<'a> VkWrappedType<RawVkImageCreateInfo> for VkImageCreateInfo<'a> {
    fn vk_to_raw(src: &VkImageCreateInfo, dst: &mut RawVkImageCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.image_type = vk_to_raw_value(&src.image_type);
        dst.format = vk_to_raw_value(&src.format);
        dst.extent = vk_to_raw_value(&src.extent);
        dst.mip_levels = vk_to_raw_value(&src.mip_levels);
        dst.array_layers = vk_to_raw_value(&src.array_layers);
        dst.samples = vk_to_raw_value(&src.samples);
        dst.tiling = vk_to_raw_value(&src.tiling);
        dst.usage = vk_to_raw_value(&src.usage);
        dst.sharing_mode = vk_to_raw_value(&src.sharing_mode);
        dst.queue_family_index_count = src.queue_family_indices.len() as u32;
        dst.queue_family_indices = new_ptr_vk_array(src.queue_family_indices);
        dst.initial_layout = vk_to_raw_value(&src.initial_layout);
    }
}

impl Default for VkImageCreateInfo<'static> {
    fn default() -> VkImageCreateInfo<'static> {
        VkImageCreateInfo {
            flags: VkImageCreateFlags::default(),
            image_type: VkImageType::default(),
            format: VkFormat::default(),
            extent: VkExtent3D::default(),
            mip_levels: 0,
            array_layers: 0,
            samples: VkSampleCountFlags::default(),
            tiling: VkImageTiling::default(),
            usage: VkImageUsageFlags::default(),
            sharing_mode: VkSharingMode::default(),
            queue_family_indices: &[],
            initial_layout: VkImageLayout::default(),
        }
    }
}

impl<'a> VkSetup for VkImageCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.extent, fn_table);
    }
}

impl VkFree for RawVkImageCreateInfo {
    fn vk_free(&mut self) {
        RawVkExtent3D::vk_free(&mut self.extent);
        free_ptr(self.queue_family_indices);
    }
}