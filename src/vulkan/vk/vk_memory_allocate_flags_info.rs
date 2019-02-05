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
use vulkan::vk::{VkMemoryAllocateFlags,RawVkMemoryAllocateFlags};

/// Wrapper for [VkMemoryAllocateFlagsInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryAllocateFlagsInfo.html).
#[derive(Debug, Clone)]
pub struct VkMemoryAllocateFlagsInfo {
    pub flags: VkMemoryAllocateFlags,
    pub device_mask: u32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryAllocateFlagsInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkMemoryAllocateFlags,
    pub device_mask: u32,
}

impl VkWrappedType<RawVkMemoryAllocateFlagsInfo> for VkMemoryAllocateFlagsInfo {
    fn vk_to_raw(src: &VkMemoryAllocateFlagsInfo, dst: &mut RawVkMemoryAllocateFlagsInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryAllocateFlagsInfo);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.device_mask = src.device_mask;
    }
}

impl VkRawType<VkMemoryAllocateFlagsInfo> for RawVkMemoryAllocateFlagsInfo {
    fn vk_to_wrapped(src: &RawVkMemoryAllocateFlagsInfo) -> VkMemoryAllocateFlagsInfo {
        VkMemoryAllocateFlagsInfo {
            flags: RawVkMemoryAllocateFlags::vk_to_wrapped(&src.flags),
            device_mask: src.device_mask,
        }
    }
}

impl Default for VkMemoryAllocateFlagsInfo {
    fn default() -> VkMemoryAllocateFlagsInfo {
        VkMemoryAllocateFlagsInfo {
            flags: Default::default(),
            device_mask: 0,
        }
    }
}

impl VkSetup for VkMemoryAllocateFlagsInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkMemoryAllocateFlagsInfo {
    fn vk_free(&self) {
        
    }
}