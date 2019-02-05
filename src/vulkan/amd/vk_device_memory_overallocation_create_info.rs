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
use vulkan::amd::{VkMemoryOverallocationBehavior,RawVkMemoryOverallocationBehavior};

/// Wrapper for [VkDeviceMemoryOverallocationCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html).
#[derive(Debug, Clone)]
pub struct VkDeviceMemoryOverallocationCreateInfo {
    pub overallocation_behavior: VkMemoryOverallocationBehavior,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceMemoryOverallocationCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub overallocation_behavior: RawVkMemoryOverallocationBehavior,
}

impl VkWrappedType<RawVkDeviceMemoryOverallocationCreateInfo> for VkDeviceMemoryOverallocationCreateInfo {
    fn vk_to_raw(src: &VkDeviceMemoryOverallocationCreateInfo, dst: &mut RawVkDeviceMemoryOverallocationCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceMemoryOverallocationCreateInfoAmd);
        dst.next = ptr::null();
        dst.overallocation_behavior = vk_to_raw_value(&src.overallocation_behavior);
    }
}

impl VkRawType<VkDeviceMemoryOverallocationCreateInfo> for RawVkDeviceMemoryOverallocationCreateInfo {
    fn vk_to_wrapped(src: &RawVkDeviceMemoryOverallocationCreateInfo) -> VkDeviceMemoryOverallocationCreateInfo {
        VkDeviceMemoryOverallocationCreateInfo {
            overallocation_behavior: RawVkMemoryOverallocationBehavior::vk_to_wrapped(&src.overallocation_behavior),
        }
    }
}

impl Default for VkDeviceMemoryOverallocationCreateInfo {
    fn default() -> VkDeviceMemoryOverallocationCreateInfo {
        VkDeviceMemoryOverallocationCreateInfo {
            overallocation_behavior: VkMemoryOverallocationBehavior::default(),
        }
    }
}

impl VkSetup for VkDeviceMemoryOverallocationCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDeviceMemoryOverallocationCreateInfo {
    fn vk_free(&mut self) {
        
    }
}