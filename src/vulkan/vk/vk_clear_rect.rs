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
use vulkan::vk::{VkRect2D,RawVkRect2D};

/// Wrapper for [VkClearRect](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearRect.html).
#[derive(Debug, Clone)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub base_array_layer: usize,
    pub layer_count: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkClearRect {
    pub rect: RawVkRect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl VkWrappedType<RawVkClearRect> for VkClearRect {
    fn vk_to_raw(src: &VkClearRect, dst: &mut RawVkClearRect) {
        dst.rect = vk_to_raw_value(&src.rect);
        dst.base_array_layer = vk_to_raw_value(&src.base_array_layer);
        dst.layer_count = vk_to_raw_value(&src.layer_count);
    }
}

impl VkRawType<VkClearRect> for RawVkClearRect {
    fn vk_to_wrapped(src: &RawVkClearRect) -> VkClearRect {
        VkClearRect {
            rect: RawVkRect2D::vk_to_wrapped(&src.rect),
            base_array_layer: u32::vk_to_wrapped(&src.base_array_layer),
            layer_count: u32::vk_to_wrapped(&src.layer_count),
        }
    }
}

impl Default for VkClearRect {
    fn default() -> VkClearRect {
        VkClearRect {
            rect: Default::default(),
            base_array_layer: 0,
            layer_count: 0,
        }
    }
}

impl VkSetup for VkClearRect {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.rect, fn_table);
    }
}

impl VkFree for RawVkClearRect {
    fn vk_free(&self) {
        
    }
}