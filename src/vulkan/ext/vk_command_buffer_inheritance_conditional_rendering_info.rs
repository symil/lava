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

/// Wrapper for [VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfo {
    pub conditional_rendering_enable: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandBufferInheritanceConditionalRenderingInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub conditional_rendering_enable: u32,
}

impl VkWrappedType<RawVkCommandBufferInheritanceConditionalRenderingInfo> for VkCommandBufferInheritanceConditionalRenderingInfo {
    fn vk_to_raw(src: &VkCommandBufferInheritanceConditionalRenderingInfo, dst: &mut RawVkCommandBufferInheritanceConditionalRenderingInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandBufferInheritanceConditionalRenderingInfoExt);
        dst.next = ptr::null_mut();
        dst.conditional_rendering_enable = vk_to_raw_value(&src.conditional_rendering_enable);
    }
}

impl VkRawType<VkCommandBufferInheritanceConditionalRenderingInfo> for RawVkCommandBufferInheritanceConditionalRenderingInfo {
    fn vk_to_wrapped(src: &RawVkCommandBufferInheritanceConditionalRenderingInfo) -> VkCommandBufferInheritanceConditionalRenderingInfo {
        VkCommandBufferInheritanceConditionalRenderingInfo {
            conditional_rendering_enable: u32::vk_to_wrapped(&src.conditional_rendering_enable),
        }
    }
}

impl Default for VkCommandBufferInheritanceConditionalRenderingInfo {
    fn default() -> VkCommandBufferInheritanceConditionalRenderingInfo {
        VkCommandBufferInheritanceConditionalRenderingInfo {
            conditional_rendering_enable: false,
        }
    }
}

impl VkSetup for VkCommandBufferInheritanceConditionalRenderingInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkCommandBufferInheritanceConditionalRenderingInfo {
    fn vk_free(&self) {
        
    }
}