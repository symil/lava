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

/// Wrapper for [VkDrawIndexedIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndexedIndirectCommand.html).
#[derive(Debug, Clone)]
pub struct VkDrawIndexedIndirectCommand {
    pub index_count: usize,
    pub instance_count: usize,
    pub first_index: usize,
    pub vertex_offset: isize,
    pub first_instance: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

impl VkWrappedType<RawVkDrawIndexedIndirectCommand> for VkDrawIndexedIndirectCommand {
    fn vk_to_raw(src: &VkDrawIndexedIndirectCommand, dst: &mut RawVkDrawIndexedIndirectCommand) {
        dst.index_count = vk_to_raw_value(&src.index_count);
        dst.instance_count = vk_to_raw_value(&src.instance_count);
        dst.first_index = vk_to_raw_value(&src.first_index);
        dst.vertex_offset = vk_to_raw_value(&src.vertex_offset);
        dst.first_instance = vk_to_raw_value(&src.first_instance);
    }
}

impl VkRawType<VkDrawIndexedIndirectCommand> for RawVkDrawIndexedIndirectCommand {
    fn vk_to_wrapped(src: &RawVkDrawIndexedIndirectCommand) -> VkDrawIndexedIndirectCommand {
        VkDrawIndexedIndirectCommand {
            index_count: u32::vk_to_wrapped(&src.index_count),
            instance_count: u32::vk_to_wrapped(&src.instance_count),
            first_index: u32::vk_to_wrapped(&src.first_index),
            vertex_offset: i32::vk_to_wrapped(&src.vertex_offset),
            first_instance: u32::vk_to_wrapped(&src.first_instance),
        }
    }
}

impl Default for VkDrawIndexedIndirectCommand {
    fn default() -> VkDrawIndexedIndirectCommand {
        VkDrawIndexedIndirectCommand {
            index_count: 0,
            instance_count: 0,
            first_index: 0,
            vertex_offset: 0,
            first_instance: 0,
        }
    }
}

impl VkSetup for VkDrawIndexedIndirectCommand {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDrawIndexedIndirectCommand {
    fn vk_free(&self) {
        
    }
}