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
use vulkan::nv::{VkViewportCoordinateSwizzle,RawVkViewportCoordinateSwizzle};

/// Wrapper for [VkViewportSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkViewportSwizzleNV.html).
#[derive(Debug, Clone)]
pub struct VkViewportSwizzle {
    pub x: VkViewportCoordinateSwizzle,
    pub y: VkViewportCoordinateSwizzle,
    pub z: VkViewportCoordinateSwizzle,
    pub w: VkViewportCoordinateSwizzle,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkViewportSwizzle {
    pub x: RawVkViewportCoordinateSwizzle,
    pub y: RawVkViewportCoordinateSwizzle,
    pub z: RawVkViewportCoordinateSwizzle,
    pub w: RawVkViewportCoordinateSwizzle,
}

impl VkWrappedType<RawVkViewportSwizzle> for VkViewportSwizzle {
    fn vk_to_raw(src: &VkViewportSwizzle, dst: &mut RawVkViewportSwizzle) {
        dst.x = vk_to_raw_value(&src.x);
        dst.y = vk_to_raw_value(&src.y);
        dst.z = vk_to_raw_value(&src.z);
        dst.w = vk_to_raw_value(&src.w);
    }
}

impl VkRawType<VkViewportSwizzle> for RawVkViewportSwizzle {
    fn vk_to_wrapped(src: &RawVkViewportSwizzle) -> VkViewportSwizzle {
        VkViewportSwizzle {
            x: RawVkViewportCoordinateSwizzle::vk_to_wrapped(&src.x),
            y: RawVkViewportCoordinateSwizzle::vk_to_wrapped(&src.y),
            z: RawVkViewportCoordinateSwizzle::vk_to_wrapped(&src.z),
            w: RawVkViewportCoordinateSwizzle::vk_to_wrapped(&src.w),
        }
    }
}

impl Default for VkViewportSwizzle {
    fn default() -> VkViewportSwizzle {
        VkViewportSwizzle {
            x: VkViewportCoordinateSwizzle::default(),
            y: VkViewportCoordinateSwizzle::default(),
            z: VkViewportCoordinateSwizzle::default(),
            w: VkViewportCoordinateSwizzle::default(),
        }
    }
}

impl VkSetup for VkViewportSwizzle {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkViewportSwizzle {
    fn vk_free(&mut self) {
        
    }
}