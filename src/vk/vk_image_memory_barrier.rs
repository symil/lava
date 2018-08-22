// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::vk_access_flags::*;
use vk::vk_image_layout::*;
use vk::vk_image::*;
use vk::vk_image_subresource_range::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageMemoryBarrier {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
    pub old_layout: RawVkImageLayout,
    pub new_layout: RawVkImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: RawVkImage,
    pub subresource_range: RawVkImageSubresourceRange,
}

#[derive(Debug, Clone)]
pub struct VkImageMemoryBarrier<'a> {
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub old_layout: VkImageLayout,
    pub new_layout: VkImageLayout,
    pub src_queue_family_index: usize,
    pub dst_queue_family_index: usize,
    pub image: &'a VkImage,
    pub subresource_range: VkImageSubresourceRange,
}

impl<'a> VkWrappedType<RawVkImageMemoryBarrier> for VkImageMemoryBarrier<'a> {
    fn vk_to_raw(src: &VkImageMemoryBarrier, dst: &mut RawVkImageMemoryBarrier) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageMemoryBarrier);
        dst.next = ptr::null();
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
        dst.old_layout = vk_to_raw_value(&src.old_layout);
        dst.new_layout = vk_to_raw_value(&src.new_layout);
        dst.src_queue_family_index = vk_to_raw_value(&src.src_queue_family_index);
        dst.dst_queue_family_index = vk_to_raw_value(&src.dst_queue_family_index);
        dst.image = vk_to_raw_value(src.image);
        dst.subresource_range = vk_to_raw_value(&src.subresource_range);
    }
}

impl Default for VkImageMemoryBarrier<'static> {
    fn default() -> VkImageMemoryBarrier<'static> {
        VkImageMemoryBarrier {
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
            old_layout: VkImageLayout::default(),
            new_layout: VkImageLayout::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            image: vk_null_ref(),
            subresource_range: VkImageSubresourceRange::default(),
        }
    }
}

impl<'a> VkSetup for VkImageMemoryBarrier<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.subresource_range, fn_table, instance, device);
    }
}

impl VkFree for RawVkImageMemoryBarrier {
    fn vk_free(&mut self) {
        RawVkImageSubresourceRange::vk_free(&mut self.subresource_range);
    }
}