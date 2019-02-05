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
use vulkan::vk::{VkSparseImageFormatProperties,RawVkSparseImageFormatProperties};

/// Wrapper for [VkSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSparseImageFormatProperties2.html).
#[derive(Debug, Clone)]
pub struct VkSparseImageFormatProperties2 {
    pub properties: VkSparseImageFormatProperties,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageFormatProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub properties: RawVkSparseImageFormatProperties,
}

impl VkWrappedType<RawVkSparseImageFormatProperties2> for VkSparseImageFormatProperties2 {
    fn vk_to_raw(src: &VkSparseImageFormatProperties2, dst: &mut RawVkSparseImageFormatProperties2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SparseImageFormatProperties2);
        dst.next = ptr::null_mut();
        dst.properties = vk_to_raw_value(&src.properties);
    }
}

impl VkRawType<VkSparseImageFormatProperties2> for RawVkSparseImageFormatProperties2 {
    fn vk_to_wrapped(src: &RawVkSparseImageFormatProperties2) -> VkSparseImageFormatProperties2 {
        VkSparseImageFormatProperties2 {
            properties: RawVkSparseImageFormatProperties::vk_to_wrapped(&src.properties),
        }
    }
}

impl Default for VkSparseImageFormatProperties2 {
    fn default() -> VkSparseImageFormatProperties2 {
        VkSparseImageFormatProperties2 {
            properties: Default::default(),
        }
    }
}

impl VkSetup for VkSparseImageFormatProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.properties, fn_table);
    }
}

impl VkFree for RawVkSparseImageFormatProperties2 {
    fn vk_free(&self) {
        
    }
}