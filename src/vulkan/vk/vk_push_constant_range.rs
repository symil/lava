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
use vulkan::vk::{VkShaderStageFlags,RawVkShaderStageFlags};

/// Wrapper for [VkPushConstantRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPushConstantRange.html).
#[derive(Debug, Clone)]
pub struct VkPushConstantRange {
    pub stage_flags: VkShaderStageFlags,
    pub offset: usize,
    pub size: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPushConstantRange {
    pub stage_flags: RawVkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

impl VkWrappedType<RawVkPushConstantRange> for VkPushConstantRange {
    fn vk_to_raw(src: &VkPushConstantRange, dst: &mut RawVkPushConstantRange) {
        dst.stage_flags = vk_to_raw_value(&src.stage_flags);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.size = vk_to_raw_value(&src.size);
    }
}

impl VkRawType<VkPushConstantRange> for RawVkPushConstantRange {
    fn vk_to_wrapped(src: &RawVkPushConstantRange) -> VkPushConstantRange {
        VkPushConstantRange {
            stage_flags: RawVkShaderStageFlags::vk_to_wrapped(&src.stage_flags),
            offset: u32::vk_to_wrapped(&src.offset),
            size: u32::vk_to_wrapped(&src.size),
        }
    }
}

impl Default for VkPushConstantRange {
    fn default() -> VkPushConstantRange {
        VkPushConstantRange {
            stage_flags: Default::default(),
            offset: 0,
            size: 0,
        }
    }
}

impl VkSetup for VkPushConstantRange {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPushConstantRange {
    fn vk_free(&self) {
        
    }
}