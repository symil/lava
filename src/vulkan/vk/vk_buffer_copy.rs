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

/// Wrapper for [VkBufferCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy.html).
#[derive(Debug, Clone)]
pub struct VkBufferCopy {
    pub src_offset: usize,
    pub dst_offset: usize,
    pub size: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferCopy {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

impl VkWrappedType<RawVkBufferCopy> for VkBufferCopy {
    fn vk_to_raw(src: &VkBufferCopy, dst: &mut RawVkBufferCopy) {
        dst.src_offset = vk_to_raw_value(&src.src_offset);
        dst.dst_offset = vk_to_raw_value(&src.dst_offset);
        dst.size = vk_to_raw_value(&src.size);
    }
}

impl VkRawType<VkBufferCopy> for RawVkBufferCopy {
    fn vk_to_wrapped(src: &RawVkBufferCopy) -> VkBufferCopy {
        VkBufferCopy {
            src_offset: u64::vk_to_wrapped(&src.src_offset),
            dst_offset: u64::vk_to_wrapped(&src.dst_offset),
            size: u64::vk_to_wrapped(&src.size),
        }
    }
}

impl Default for VkBufferCopy {
    fn default() -> VkBufferCopy {
        VkBufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: 0,
        }
    }
}

impl VkSetup for VkBufferCopy {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkBufferCopy {
    fn vk_free(&self) {
        
    }
}