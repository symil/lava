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
use vulkan::vk::{VkPipelineStageFlags,RawVkPipelineStageFlags};

/// Wrapper for [VkCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointDataNV.html).
#[derive(Debug, Clone)]
pub struct VkCheckpointData {
    pub stage: VkPipelineStageFlags,
    pub checkpoint_marker: *mut c_void,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCheckpointData {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub stage: RawVkPipelineStageFlags,
    pub checkpoint_marker: *mut c_void,
}

impl VkWrappedType<RawVkCheckpointData> for VkCheckpointData {
    fn vk_to_raw(src: &VkCheckpointData, dst: &mut RawVkCheckpointData) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CheckpointDataNv);
        dst.next = ptr::null_mut();
        dst.stage = vk_to_raw_value(&src.stage);
        dst.checkpoint_marker = src.checkpoint_marker;
    }
}

impl VkRawType<VkCheckpointData> for RawVkCheckpointData {
    fn vk_to_wrapped(src: &RawVkCheckpointData) -> VkCheckpointData {
        VkCheckpointData {
            stage: RawVkPipelineStageFlags::vk_to_wrapped(&src.stage),
            checkpoint_marker: src.checkpoint_marker,
        }
    }
}

impl Default for VkCheckpointData {
    fn default() -> VkCheckpointData {
        VkCheckpointData {
            stage: Default::default(),
            checkpoint_marker: ptr::null_mut(),
        }
    }
}

impl VkSetup for VkCheckpointData {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkCheckpointData {
    fn vk_free(&self) {
        
    }
}