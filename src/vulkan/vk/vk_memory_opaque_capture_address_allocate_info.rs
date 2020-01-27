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

/// Wrapper for [VkMemoryOpaqueCaptureAddressAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html).
#[derive(Debug, Clone)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
    pub opaque_capture_address: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub opaque_capture_address: u64,
}

impl VkWrappedType<RawVkMemoryOpaqueCaptureAddressAllocateInfo> for VkMemoryOpaqueCaptureAddressAllocateInfo {
    fn vk_to_raw(src: &VkMemoryOpaqueCaptureAddressAllocateInfo, dst: &mut RawVkMemoryOpaqueCaptureAddressAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryOpaqueCaptureAddressAllocateInfo);
        dst.next = ptr::null_mut();
        dst.opaque_capture_address = vk_to_raw_value(&src.opaque_capture_address);
    }
}

impl VkRawType<VkMemoryOpaqueCaptureAddressAllocateInfo> for RawVkMemoryOpaqueCaptureAddressAllocateInfo {
    fn vk_to_wrapped(src: &RawVkMemoryOpaqueCaptureAddressAllocateInfo) -> VkMemoryOpaqueCaptureAddressAllocateInfo {
        VkMemoryOpaqueCaptureAddressAllocateInfo {
            opaque_capture_address: u64::vk_to_wrapped(&src.opaque_capture_address),
        }
    }
}

impl Default for VkMemoryOpaqueCaptureAddressAllocateInfo {
    fn default() -> VkMemoryOpaqueCaptureAddressAllocateInfo {
        VkMemoryOpaqueCaptureAddressAllocateInfo {
            opaque_capture_address: 0,
        }
    }
}

impl VkSetup for VkMemoryOpaqueCaptureAddressAllocateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkMemoryOpaqueCaptureAddressAllocateInfo {
    fn vk_free(&self) {
        
    }
}