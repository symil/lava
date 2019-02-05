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

/// Wrapper for [VkRenderPassMultiviewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkRenderPassMultiviewCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkRenderPassMultiviewCreateInfo<'a, 'b, 'c> {
    pub view_masks: &'a [u32],
    pub view_offsets: &'b [isize],
    pub correlation_masks: &'c [u32],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkRenderPassMultiviewCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub subpass_count: u32,
    pub view_masks: *const u32,
    pub dependency_count: u32,
    pub view_offsets: *mut i32,
    pub correlation_mask_count: u32,
    pub correlation_masks: *const u32,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkRenderPassMultiviewCreateInfo> for VkRenderPassMultiviewCreateInfo<'a, 'b, 'c> {
    fn vk_to_raw(src: &VkRenderPassMultiviewCreateInfo, dst: &mut RawVkRenderPassMultiviewCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::RenderPassMultiviewCreateInfo);
        dst.next = ptr::null();
        dst.subpass_count = src.view_masks.len() as u32;
        dst.view_masks = src.view_masks.as_ptr();
        dst.dependency_count = src.view_offsets.len() as u32;
        dst.view_offsets = new_ptr_vk_array(src.view_offsets);
        dst.correlation_mask_count = src.correlation_masks.len() as u32;
        dst.correlation_masks = src.correlation_masks.as_ptr();
    }
}

impl Default for VkRenderPassMultiviewCreateInfo<'static, 'static, 'static> {
    fn default() -> VkRenderPassMultiviewCreateInfo<'static, 'static, 'static> {
        VkRenderPassMultiviewCreateInfo {
            view_masks: &[],
            view_offsets: &[],
            correlation_masks: &[],
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkRenderPassMultiviewCreateInfo<'a, 'b, 'c> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkRenderPassMultiviewCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.view_offsets);
    }
}