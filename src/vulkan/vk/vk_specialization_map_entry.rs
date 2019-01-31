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

/// Wrapper for [VkSpecializationMapEntry](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSpecializationMapEntry.html)
#[derive(Debug, Clone)]
pub struct VkSpecializationMapEntry {
    pub constant_id: usize,
    pub offset: usize,
    pub size: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

impl VkWrappedType<RawVkSpecializationMapEntry> for VkSpecializationMapEntry {
    fn vk_to_raw(src: &VkSpecializationMapEntry, dst: &mut RawVkSpecializationMapEntry) {
        dst.constant_id = vk_to_raw_value(&src.constant_id);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.size = src.size;
    }
}

impl VkRawType<VkSpecializationMapEntry> for RawVkSpecializationMapEntry {
    fn vk_to_wrapped(src: &RawVkSpecializationMapEntry) -> VkSpecializationMapEntry {
        VkSpecializationMapEntry {
            constant_id: u32::vk_to_wrapped(&src.constant_id),
            offset: u32::vk_to_wrapped(&src.offset),
            size: src.size,
        }
    }
}

impl Default for VkSpecializationMapEntry {
    fn default() -> VkSpecializationMapEntry {
        VkSpecializationMapEntry {
            constant_id: 0,
            offset: 0,
            size: 0,
        }
    }
}

impl VkSetup for VkSpecializationMapEntry {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSpecializationMapEntry {
    fn vk_free(&mut self) {
        
    }
}