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

/// Wrapper for [VkViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkViewportWScalingNV.html).
#[derive(Debug, Clone)]
pub struct VkViewportWScaling {
    pub xcoeff: f32,
    pub ycoeff: f32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkViewportWScaling {
    pub xcoeff: f32,
    pub ycoeff: f32,
}

impl VkWrappedType<RawVkViewportWScaling> for VkViewportWScaling {
    fn vk_to_raw(src: &VkViewportWScaling, dst: &mut RawVkViewportWScaling) {
        dst.xcoeff = src.xcoeff;
        dst.ycoeff = src.ycoeff;
    }
}

impl VkRawType<VkViewportWScaling> for RawVkViewportWScaling {
    fn vk_to_wrapped(src: &RawVkViewportWScaling) -> VkViewportWScaling {
        VkViewportWScaling {
            xcoeff: src.xcoeff,
            ycoeff: src.ycoeff,
        }
    }
}

impl Default for VkViewportWScaling {
    fn default() -> VkViewportWScaling {
        VkViewportWScaling {
            xcoeff: 0.0,
            ycoeff: 0.0,
        }
    }
}

impl VkSetup for VkViewportWScaling {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkViewportWScaling {
    fn vk_free(&self) {
        
    }
}