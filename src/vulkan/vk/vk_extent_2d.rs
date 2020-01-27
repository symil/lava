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

/// Wrapper for [VkExtent2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html).
#[derive(Debug, Clone)]
pub struct VkExtent2D {
    pub width: usize,
    pub height: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExtent2D {
    pub width: u32,
    pub height: u32,
}

impl VkWrappedType<RawVkExtent2D> for VkExtent2D {
    fn vk_to_raw(src: &VkExtent2D, dst: &mut RawVkExtent2D) {
        dst.width = vk_to_raw_value(&src.width);
        dst.height = vk_to_raw_value(&src.height);
    }
}

impl VkRawType<VkExtent2D> for RawVkExtent2D {
    fn vk_to_wrapped(src: &RawVkExtent2D) -> VkExtent2D {
        VkExtent2D {
            width: u32::vk_to_wrapped(&src.width),
            height: u32::vk_to_wrapped(&src.height),
        }
    }
}

impl Default for VkExtent2D {
    fn default() -> VkExtent2D {
        VkExtent2D {
            width: 0,
            height: 0,
        }
    }
}

impl VkSetup for VkExtent2D {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkExtent2D {
    fn vk_free(&self) {
        
    }
}