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
use vulkan::vk::{VkAccessFlags,RawVkAccessFlags};

/// Wrapper for [VkMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryBarrier.html).
#[derive(Debug, Clone)]
pub struct VkMemoryBarrier {
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryBarrier {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
}

impl VkWrappedType<RawVkMemoryBarrier> for VkMemoryBarrier {
    fn vk_to_raw(src: &VkMemoryBarrier, dst: &mut RawVkMemoryBarrier) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryBarrier);
        dst.next = ptr::null_mut();
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
    }
}

impl VkRawType<VkMemoryBarrier> for RawVkMemoryBarrier {
    fn vk_to_wrapped(src: &RawVkMemoryBarrier) -> VkMemoryBarrier {
        VkMemoryBarrier {
            src_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.src_access_mask),
            dst_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.dst_access_mask),
        }
    }
}

impl Default for VkMemoryBarrier {
    fn default() -> VkMemoryBarrier {
        VkMemoryBarrier {
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
        }
    }
}

impl VkSetup for VkMemoryBarrier {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkMemoryBarrier {
    fn vk_free(&self) {
        
    }
}