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

/// Wrapper for [VkPresentTimeGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPresentTimeGOOGLE.html).
#[derive(Debug, Clone)]
pub struct VkPresentTime {
    pub present_id: usize,
    pub desired_present_time: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPresentTime {
    pub present_id: u32,
    pub desired_present_time: u64,
}

impl VkWrappedType<RawVkPresentTime> for VkPresentTime {
    fn vk_to_raw(src: &VkPresentTime, dst: &mut RawVkPresentTime) {
        dst.present_id = vk_to_raw_value(&src.present_id);
        dst.desired_present_time = vk_to_raw_value(&src.desired_present_time);
    }
}

impl VkRawType<VkPresentTime> for RawVkPresentTime {
    fn vk_to_wrapped(src: &RawVkPresentTime) -> VkPresentTime {
        VkPresentTime {
            present_id: u32::vk_to_wrapped(&src.present_id),
            desired_present_time: u64::vk_to_wrapped(&src.desired_present_time),
        }
    }
}

impl Default for VkPresentTime {
    fn default() -> VkPresentTime {
        VkPresentTime {
            present_id: 0,
            desired_present_time: 0,
        }
    }
}

impl VkSetup for VkPresentTime {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPresentTime {
    fn vk_free(&mut self) {
        
    }
}