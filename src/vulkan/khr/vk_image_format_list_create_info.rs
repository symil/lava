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
use vulkan::vk::{VkFormat,RawVkFormat};

/// Wrapper for [VkImageFormatListCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageFormatListCreateInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkImageFormatListCreateInfo {
    pub view_formats: Vec<VkFormat>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageFormatListCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub view_format_count: u32,
    pub view_formats: *mut RawVkFormat,
}

impl VkWrappedType<RawVkImageFormatListCreateInfo> for VkImageFormatListCreateInfo {
    fn vk_to_raw(src: &VkImageFormatListCreateInfo, dst: &mut RawVkImageFormatListCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageFormatListCreateInfoKhr);
        dst.next = ptr::null_mut();
        dst.view_format_count = src.view_formats.len() as u32;
        dst.view_formats = new_ptr_vk_array(&src.view_formats);
    }
}

impl VkRawType<VkImageFormatListCreateInfo> for RawVkImageFormatListCreateInfo {
    fn vk_to_wrapped(src: &RawVkImageFormatListCreateInfo) -> VkImageFormatListCreateInfo {
        VkImageFormatListCreateInfo {
            view_formats: new_vk_array(src.view_format_count, src.view_formats),
        }
    }
}

impl Default for VkImageFormatListCreateInfo {
    fn default() -> VkImageFormatListCreateInfo {
        VkImageFormatListCreateInfo {
            view_formats: Vec::new(),
        }
    }
}

impl VkSetup for VkImageFormatListCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkImageFormatListCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.view_formats);
    }
}