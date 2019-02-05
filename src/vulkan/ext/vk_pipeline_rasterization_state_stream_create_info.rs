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
use vulkan::ext::{VkPipelineRasterizationStateStreamCreateFlags,RawVkPipelineRasterizationStateStreamCreateFlags};

/// Wrapper for [VkPipelineRasterizationStateStreamCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationStateStreamCreateInfo {
    pub flags: VkPipelineRasterizationStateStreamCreateFlags,
    pub rasterization_stream: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineRasterizationStateStreamCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineRasterizationStateStreamCreateFlags,
    pub rasterization_stream: u32,
}

impl VkWrappedType<RawVkPipelineRasterizationStateStreamCreateInfo> for VkPipelineRasterizationStateStreamCreateInfo {
    fn vk_to_raw(src: &VkPipelineRasterizationStateStreamCreateInfo, dst: &mut RawVkPipelineRasterizationStateStreamCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineRasterizationStateStreamCreateInfoExt);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.rasterization_stream = vk_to_raw_value(&src.rasterization_stream);
    }
}

impl VkRawType<VkPipelineRasterizationStateStreamCreateInfo> for RawVkPipelineRasterizationStateStreamCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationStateStreamCreateInfo) -> VkPipelineRasterizationStateStreamCreateInfo {
        VkPipelineRasterizationStateStreamCreateInfo {
            flags: RawVkPipelineRasterizationStateStreamCreateFlags::vk_to_wrapped(&src.flags),
            rasterization_stream: u32::vk_to_wrapped(&src.rasterization_stream),
        }
    }
}

impl Default for VkPipelineRasterizationStateStreamCreateInfo {
    fn default() -> VkPipelineRasterizationStateStreamCreateInfo {
        VkPipelineRasterizationStateStreamCreateInfo {
            flags: VkPipelineRasterizationStateStreamCreateFlags::default(),
            rasterization_stream: 0,
        }
    }
}

impl VkSetup for VkPipelineRasterizationStateStreamCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineRasterizationStateStreamCreateInfo {
    fn vk_free(&mut self) {
        
    }
}