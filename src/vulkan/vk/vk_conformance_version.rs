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

/// Wrapper for [VkConformanceVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConformanceVersion.html).
#[derive(Debug, Clone)]
pub struct VkConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}

impl VkWrappedType<RawVkConformanceVersion> for VkConformanceVersion {
    fn vk_to_raw(src: &VkConformanceVersion, dst: &mut RawVkConformanceVersion) {
        dst.major = src.major;
        dst.minor = src.minor;
        dst.subminor = src.subminor;
        dst.patch = src.patch;
    }
}

impl VkRawType<VkConformanceVersion> for RawVkConformanceVersion {
    fn vk_to_wrapped(src: &RawVkConformanceVersion) -> VkConformanceVersion {
        VkConformanceVersion {
            major: src.major,
            minor: src.minor,
            subminor: src.subminor,
            patch: src.patch,
        }
    }
}

impl Default for VkConformanceVersion {
    fn default() -> VkConformanceVersion {
        VkConformanceVersion {
            major: 0,
            minor: 0,
            subminor: 0,
            patch: 0,
        }
    }
}

impl VkSetup for VkConformanceVersion {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkConformanceVersion {
    fn vk_free(&self) {
        
    }
}