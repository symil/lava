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
use vulkan::ext::{VkSurfaceCounterFlags,RawVkSurfaceCounterFlags};

/// Wrapper for [VkSwapchainCounterCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkSwapchainCounterCreateInfo {
    pub surface_counters: VkSurfaceCounterFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSwapchainCounterCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub surface_counters: RawVkSurfaceCounterFlags,
}

impl VkWrappedType<RawVkSwapchainCounterCreateInfo> for VkSwapchainCounterCreateInfo {
    fn vk_to_raw(src: &VkSwapchainCounterCreateInfo, dst: &mut RawVkSwapchainCounterCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SwapchainCounterCreateInfoExt);
        dst.next = ptr::null_mut();
        dst.surface_counters = vk_to_raw_value(&src.surface_counters);
    }
}

impl VkRawType<VkSwapchainCounterCreateInfo> for RawVkSwapchainCounterCreateInfo {
    fn vk_to_wrapped(src: &RawVkSwapchainCounterCreateInfo) -> VkSwapchainCounterCreateInfo {
        VkSwapchainCounterCreateInfo {
            surface_counters: RawVkSurfaceCounterFlags::vk_to_wrapped(&src.surface_counters),
        }
    }
}

impl Default for VkSwapchainCounterCreateInfo {
    fn default() -> VkSwapchainCounterCreateInfo {
        VkSwapchainCounterCreateInfo {
            surface_counters: Default::default(),
        }
    }
}

impl VkSetup for VkSwapchainCounterCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSwapchainCounterCreateInfo {
    fn vk_free(&self) {
        
    }
}