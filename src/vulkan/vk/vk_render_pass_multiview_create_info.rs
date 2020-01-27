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

/// Wrapper for [VkRenderPassMultiviewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkRenderPassMultiviewCreateInfo {
    pub view_masks: Vec<u32>,
    pub view_offsets: Vec<isize>,
    pub correlation_masks: Vec<u32>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkRenderPassMultiviewCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub subpass_count: u32,
    pub view_masks: *mut u32,
    pub dependency_count: u32,
    pub view_offsets: *mut i32,
    pub correlation_mask_count: u32,
    pub correlation_masks: *mut u32,
}

impl VkWrappedType<RawVkRenderPassMultiviewCreateInfo> for VkRenderPassMultiviewCreateInfo {
    fn vk_to_raw(src: &VkRenderPassMultiviewCreateInfo, dst: &mut RawVkRenderPassMultiviewCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::RenderPassMultiviewCreateInfo);
        dst.next = ptr::null_mut();
        dst.subpass_count = src.view_masks.len() as u32;
        dst.view_masks = get_vec_ptr(&src.view_masks);
        dst.dependency_count = src.view_offsets.len() as u32;
        dst.view_offsets = new_ptr_vk_array(&src.view_offsets);
        dst.correlation_mask_count = src.correlation_masks.len() as u32;
        dst.correlation_masks = get_vec_ptr(&src.correlation_masks);
    }
}

impl VkRawType<VkRenderPassMultiviewCreateInfo> for RawVkRenderPassMultiviewCreateInfo {
    fn vk_to_wrapped(src: &RawVkRenderPassMultiviewCreateInfo) -> VkRenderPassMultiviewCreateInfo {
        VkRenderPassMultiviewCreateInfo {
            view_masks: vec_from_ptr(src.subpass_count as usize, src.view_masks),
            view_offsets: new_vk_array(src.dependency_count, src.view_offsets),
            correlation_masks: vec_from_ptr(src.correlation_mask_count as usize, src.correlation_masks),
        }
    }
}

impl Default for VkRenderPassMultiviewCreateInfo {
    fn default() -> VkRenderPassMultiviewCreateInfo {
        VkRenderPassMultiviewCreateInfo {
            view_masks: Vec::new(),
            view_offsets: Vec::new(),
            correlation_masks: Vec::new(),
        }
    }
}

impl VkSetup for VkRenderPassMultiviewCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkRenderPassMultiviewCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.view_offsets);
    }
}