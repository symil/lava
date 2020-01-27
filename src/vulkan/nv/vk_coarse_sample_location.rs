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

/// Wrapper for [VkCoarseSampleLocationNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleLocationNV.html).
#[derive(Debug, Clone)]
pub struct VkCoarseSampleLocation {
    pub pixel_x: usize,
    pub pixel_y: usize,
    pub sample: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCoarseSampleLocation {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}

impl VkWrappedType<RawVkCoarseSampleLocation> for VkCoarseSampleLocation {
    fn vk_to_raw(src: &VkCoarseSampleLocation, dst: &mut RawVkCoarseSampleLocation) {
        dst.pixel_x = vk_to_raw_value(&src.pixel_x);
        dst.pixel_y = vk_to_raw_value(&src.pixel_y);
        dst.sample = vk_to_raw_value(&src.sample);
    }
}

impl VkRawType<VkCoarseSampleLocation> for RawVkCoarseSampleLocation {
    fn vk_to_wrapped(src: &RawVkCoarseSampleLocation) -> VkCoarseSampleLocation {
        VkCoarseSampleLocation {
            pixel_x: u32::vk_to_wrapped(&src.pixel_x),
            pixel_y: u32::vk_to_wrapped(&src.pixel_y),
            sample: u32::vk_to_wrapped(&src.sample),
        }
    }
}

impl Default for VkCoarseSampleLocation {
    fn default() -> VkCoarseSampleLocation {
        VkCoarseSampleLocation {
            pixel_x: 0,
            pixel_y: 0,
            sample: 0,
        }
    }
}

impl VkSetup for VkCoarseSampleLocation {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkCoarseSampleLocation {
    fn vk_free(&self) {
        
    }
}