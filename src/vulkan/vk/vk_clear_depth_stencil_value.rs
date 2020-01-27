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

/// Wrapper for [VkClearDepthStencilValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearDepthStencilValue.html).
#[derive(Debug, Clone)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

impl VkWrappedType<RawVkClearDepthStencilValue> for VkClearDepthStencilValue {
    fn vk_to_raw(src: &VkClearDepthStencilValue, dst: &mut RawVkClearDepthStencilValue) {
        dst.depth = src.depth;
        dst.stencil = vk_to_raw_value(&src.stencil);
    }
}

impl VkRawType<VkClearDepthStencilValue> for RawVkClearDepthStencilValue {
    fn vk_to_wrapped(src: &RawVkClearDepthStencilValue) -> VkClearDepthStencilValue {
        VkClearDepthStencilValue {
            depth: src.depth,
            stencil: u32::vk_to_wrapped(&src.stencil),
        }
    }
}

impl Default for VkClearDepthStencilValue {
    fn default() -> VkClearDepthStencilValue {
        VkClearDepthStencilValue {
            depth: 0.0,
            stencil: 0,
        }
    }
}

impl VkSetup for VkClearDepthStencilValue {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkClearDepthStencilValue {
    fn vk_free(&self) {
        
    }
}