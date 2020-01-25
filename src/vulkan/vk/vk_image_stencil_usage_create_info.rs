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
use vulkan::vk::{VkImageUsageFlags,RawVkImageUsageFlags};

/// Wrapper for [VkImageStencilUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageStencilUsageCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkImageStencilUsageCreateInfo {
    pub stencil_usage: VkImageUsageFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageStencilUsageCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub stencil_usage: RawVkImageUsageFlags,
}

impl VkWrappedType<RawVkImageStencilUsageCreateInfo> for VkImageStencilUsageCreateInfo {
    fn vk_to_raw(src: &VkImageStencilUsageCreateInfo, dst: &mut RawVkImageStencilUsageCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageStencilUsageCreateInfo);
        dst.next = ptr::null_mut();
        dst.stencil_usage = vk_to_raw_value(&src.stencil_usage);
    }
}

impl VkRawType<VkImageStencilUsageCreateInfo> for RawVkImageStencilUsageCreateInfo {
    fn vk_to_wrapped(src: &RawVkImageStencilUsageCreateInfo) -> VkImageStencilUsageCreateInfo {
        VkImageStencilUsageCreateInfo {
            stencil_usage: RawVkImageUsageFlags::vk_to_wrapped(&src.stencil_usage),
        }
    }
}

impl Default for VkImageStencilUsageCreateInfo {
    fn default() -> VkImageStencilUsageCreateInfo {
        VkImageStencilUsageCreateInfo {
            stencil_usage: Default::default(),
        }
    }
}

impl VkSetup for VkImageStencilUsageCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkImageStencilUsageCreateInfo {
    fn vk_free(&self) {
        
    }
}