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

/// Wrapper for [VkDrawIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndirectCommand.html).
#[derive(Debug, Clone)]
pub struct VkDrawIndirectCommand {
    pub vertex_count: usize,
    pub instance_count: usize,
    pub first_vertex: usize,
    pub first_instance: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

impl VkWrappedType<RawVkDrawIndirectCommand> for VkDrawIndirectCommand {
    fn vk_to_raw(src: &VkDrawIndirectCommand, dst: &mut RawVkDrawIndirectCommand) {
        dst.vertex_count = vk_to_raw_value(&src.vertex_count);
        dst.instance_count = vk_to_raw_value(&src.instance_count);
        dst.first_vertex = vk_to_raw_value(&src.first_vertex);
        dst.first_instance = vk_to_raw_value(&src.first_instance);
    }
}

impl VkRawType<VkDrawIndirectCommand> for RawVkDrawIndirectCommand {
    fn vk_to_wrapped(src: &RawVkDrawIndirectCommand) -> VkDrawIndirectCommand {
        VkDrawIndirectCommand {
            vertex_count: u32::vk_to_wrapped(&src.vertex_count),
            instance_count: u32::vk_to_wrapped(&src.instance_count),
            first_vertex: u32::vk_to_wrapped(&src.first_vertex),
            first_instance: u32::vk_to_wrapped(&src.first_instance),
        }
    }
}

impl Default for VkDrawIndirectCommand {
    fn default() -> VkDrawIndirectCommand {
        VkDrawIndirectCommand {
            vertex_count: 0,
            instance_count: 0,
            first_vertex: 0,
            first_instance: 0,
        }
    }
}

impl VkSetup for VkDrawIndirectCommand {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDrawIndirectCommand {
    fn vk_free(&self) {
        
    }
}