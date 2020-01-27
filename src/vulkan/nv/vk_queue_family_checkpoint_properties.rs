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

/// Wrapper for [VkQueueFamilyCheckpointPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html).
#[derive(Debug, Clone)]
pub struct VkQueueFamilyCheckpointProperties {
    pub checkpoint_execution_stage_mask: VkPipelineStageFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkQueueFamilyCheckpointProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub checkpoint_execution_stage_mask: RawVkPipelineStageFlags,
}

impl VkWrappedType<RawVkQueueFamilyCheckpointProperties> for VkQueueFamilyCheckpointProperties {
    fn vk_to_raw(src: &VkQueueFamilyCheckpointProperties, dst: &mut RawVkQueueFamilyCheckpointProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::QueueFamilyCheckpointPropertiesNv);
        dst.next = ptr::null_mut();
        dst.checkpoint_execution_stage_mask = vk_to_raw_value(&src.checkpoint_execution_stage_mask);
    }
}

impl VkRawType<VkQueueFamilyCheckpointProperties> for RawVkQueueFamilyCheckpointProperties {
    fn vk_to_wrapped(src: &RawVkQueueFamilyCheckpointProperties) -> VkQueueFamilyCheckpointProperties {
        VkQueueFamilyCheckpointProperties {
            checkpoint_execution_stage_mask: RawVkPipelineStageFlags::vk_to_wrapped(&src.checkpoint_execution_stage_mask),
        }
    }
}

impl Default for VkQueueFamilyCheckpointProperties {
    fn default() -> VkQueueFamilyCheckpointProperties {
        VkQueueFamilyCheckpointProperties {
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}

impl VkSetup for VkQueueFamilyCheckpointProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkQueueFamilyCheckpointProperties {
    fn vk_free(&self) {
        
    }
}