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
use vulkan::vk::{VkImage,RawVkImage};
use vulkan::vk::{VkBuffer,RawVkBuffer};

/// Wrapper for [VkMemoryDedicatedAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryDedicatedAllocateInfo.html).
#[derive(Debug, Clone)]
pub struct VkMemoryDedicatedAllocateInfo {
    pub image: Option<VkImage>,
    pub buffer: Option<VkBuffer>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryDedicatedAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub image: RawVkImage,
    pub buffer: RawVkBuffer,
}

impl VkWrappedType<RawVkMemoryDedicatedAllocateInfo> for VkMemoryDedicatedAllocateInfo {
    fn vk_to_raw(src: &VkMemoryDedicatedAllocateInfo, dst: &mut RawVkMemoryDedicatedAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryDedicatedAllocateInfo);
        dst.next = ptr::null_mut();
        dst.image = vk_to_raw_value_checked(&src.image);
        dst.buffer = vk_to_raw_value_checked(&src.buffer);
    }
}

impl VkRawType<VkMemoryDedicatedAllocateInfo> for RawVkMemoryDedicatedAllocateInfo {
    fn vk_to_wrapped(src: &RawVkMemoryDedicatedAllocateInfo) -> VkMemoryDedicatedAllocateInfo {
        VkMemoryDedicatedAllocateInfo {
            image: Some(RawVkImage::vk_to_wrapped(&src.image)),
            buffer: Some(RawVkBuffer::vk_to_wrapped(&src.buffer)),
        }
    }
}

impl Default for VkMemoryDedicatedAllocateInfo {
    fn default() -> VkMemoryDedicatedAllocateInfo {
        VkMemoryDedicatedAllocateInfo {
            image: None,
            buffer: None,
        }
    }
}

impl VkSetup for VkMemoryDedicatedAllocateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkMemoryDedicatedAllocateInfo {
    fn vk_free(&self) {
        
    }
}