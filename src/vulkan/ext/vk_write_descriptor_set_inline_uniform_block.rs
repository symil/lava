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

/// Wrapper for [VkWriteDescriptorSetInlineUniformBlockEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetInlineUniformBlockEXT.html).
#[derive(Debug, Clone)]
pub struct VkWriteDescriptorSetInlineUniformBlock<'a> {
    pub data: &'a [c_void],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkWriteDescriptorSetInlineUniformBlock {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub data_size: u32,
    pub data: *mut c_void,
}

impl<'a> VkWrappedType<RawVkWriteDescriptorSetInlineUniformBlock> for VkWriteDescriptorSetInlineUniformBlock<'a> {
    fn vk_to_raw(src: &VkWriteDescriptorSetInlineUniformBlock, dst: &mut RawVkWriteDescriptorSetInlineUniformBlock) {
        dst.s_type = vk_to_raw_value(&VkStructureType::WriteDescriptorSetInlineUniformBlockExt);
        dst.next = ptr::null_mut();
        dst.data_size = src.data.len() as u32;
        dst.data = get_vec_ptr(src.data);
    }
}

impl<'a> VkRawType<VkWriteDescriptorSetInlineUniformBlock<'a>> for RawVkWriteDescriptorSetInlineUniformBlock {
    fn vk_to_wrapped(src: &RawVkWriteDescriptorSetInlineUniformBlock) -> VkWriteDescriptorSetInlineUniformBlock<'a> {
        VkWriteDescriptorSetInlineUniformBlock {
            data: slice_from_ptr(src.data_size as usize, src.data),
        }
    }
}

impl Default for VkWriteDescriptorSetInlineUniformBlock<'static> {
    fn default() -> VkWriteDescriptorSetInlineUniformBlock<'static> {
        VkWriteDescriptorSetInlineUniformBlock {
            data: &[],
        }
    }
}

impl<'a> VkSetup for VkWriteDescriptorSetInlineUniformBlock<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkWriteDescriptorSetInlineUniformBlock {
    fn vk_free(&self) {
        
    }
}