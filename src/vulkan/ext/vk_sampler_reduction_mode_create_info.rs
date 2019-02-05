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
use vulkan::ext::{VkSamplerReductionMode,RawVkSamplerReductionMode};

/// Wrapper for [VkSamplerReductionModeCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSamplerReductionModeCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkSamplerReductionModeCreateInfo {
    pub reduction_mode: VkSamplerReductionMode,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSamplerReductionModeCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub reduction_mode: RawVkSamplerReductionMode,
}

impl VkWrappedType<RawVkSamplerReductionModeCreateInfo> for VkSamplerReductionModeCreateInfo {
    fn vk_to_raw(src: &VkSamplerReductionModeCreateInfo, dst: &mut RawVkSamplerReductionModeCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SamplerReductionModeCreateInfoExt);
        dst.next = ptr::null_mut();
        dst.reduction_mode = vk_to_raw_value(&src.reduction_mode);
    }
}

impl VkRawType<VkSamplerReductionModeCreateInfo> for RawVkSamplerReductionModeCreateInfo {
    fn vk_to_wrapped(src: &RawVkSamplerReductionModeCreateInfo) -> VkSamplerReductionModeCreateInfo {
        VkSamplerReductionModeCreateInfo {
            reduction_mode: RawVkSamplerReductionMode::vk_to_wrapped(&src.reduction_mode),
        }
    }
}

impl Default for VkSamplerReductionModeCreateInfo {
    fn default() -> VkSamplerReductionModeCreateInfo {
        VkSamplerReductionModeCreateInfo {
            reduction_mode: Default::default(),
        }
    }
}

impl VkSetup for VkSamplerReductionModeCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSamplerReductionModeCreateInfo {
    fn vk_free(&self) {
        
    }
}