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
use vulkan::vk::{VkOffset2D,RawVkOffset2D};
use vulkan::vk::{VkExtent2D,RawVkExtent2D};

/// Wrapper for [VkRectLayerKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRectLayerKHR.html).
#[derive(Debug, Clone)]
pub struct VkRectLayer {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
    pub layer: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkRectLayer {
    pub offset: RawVkOffset2D,
    pub extent: RawVkExtent2D,
    pub layer: u32,
}

impl VkWrappedType<RawVkRectLayer> for VkRectLayer {
    fn vk_to_raw(src: &VkRectLayer, dst: &mut RawVkRectLayer) {
        dst.offset = vk_to_raw_value(&src.offset);
        dst.extent = vk_to_raw_value(&src.extent);
        dst.layer = vk_to_raw_value(&src.layer);
    }
}

impl VkRawType<VkRectLayer> for RawVkRectLayer {
    fn vk_to_wrapped(src: &RawVkRectLayer) -> VkRectLayer {
        VkRectLayer {
            offset: RawVkOffset2D::vk_to_wrapped(&src.offset),
            extent: RawVkExtent2D::vk_to_wrapped(&src.extent),
            layer: u32::vk_to_wrapped(&src.layer),
        }
    }
}

impl Default for VkRectLayer {
    fn default() -> VkRectLayer {
        VkRectLayer {
            offset: Default::default(),
            extent: Default::default(),
            layer: 0,
        }
    }
}

impl VkSetup for VkRectLayer {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.offset, fn_table);
        VkSetup::vk_setup(&mut self.extent, fn_table);
    }
}

impl VkFree for RawVkRectLayer {
    fn vk_free(&self) {
        
    }
}