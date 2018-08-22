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
use vk::vk_pipeline_stage_flags::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCheckpointData {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub stage: RawVkPipelineStageFlags,
    pub checkpoint_marker: *const c_void,
}

#[derive(Debug, Clone)]
pub struct VkCheckpointData {
    pub stage: VkPipelineStageFlags,
    pub checkpoint_marker: *const c_void,
}

impl VkRawType<VkCheckpointData> for RawVkCheckpointData {
    fn vk_to_wrapped(src: &RawVkCheckpointData) -> VkCheckpointData {
        VkCheckpointData {
            stage: RawVkPipelineStageFlags::vk_to_wrapped(&src.stage),
            checkpoint_marker: src.checkpoint_marker,
        }
    }
}

impl VkWrappedType<RawVkCheckpointData> for VkCheckpointData {
    fn vk_to_raw(src: &VkCheckpointData, dst: &mut RawVkCheckpointData) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CheckpointDataNv);
        dst.next = ptr::null();
        dst.stage = vk_to_raw_value(&src.stage);
        dst.checkpoint_marker = src.checkpoint_marker;
    }
}

impl Default for VkCheckpointData {
    fn default() -> VkCheckpointData {
        VkCheckpointData {
            stage: VkPipelineStageFlags::default(),
            checkpoint_marker: ptr::null(),
        }
    }
}

impl VkSetup for VkCheckpointData {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCheckpointData {
    fn vk_free(&mut self) {
        
    }
}