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

/// Wrapper for [VkPhysicalDeviceInlineUniformBlockFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
    pub inline_uniform_block: bool,
    pub descriptor_binding_inline_uniform_block_update_after_bind: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceInlineUniformBlockFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub inline_uniform_block: u32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceInlineUniformBlockFeatures> for VkPhysicalDeviceInlineUniformBlockFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceInlineUniformBlockFeatures, dst: &mut RawVkPhysicalDeviceInlineUniformBlockFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceInlineUniformBlockFeaturesExt);
        dst.next = ptr::null();
        dst.inline_uniform_block = vk_to_raw_value(&src.inline_uniform_block);
        dst.descriptor_binding_inline_uniform_block_update_after_bind = vk_to_raw_value(&src.descriptor_binding_inline_uniform_block_update_after_bind);
    }
}

impl VkRawType<VkPhysicalDeviceInlineUniformBlockFeatures> for RawVkPhysicalDeviceInlineUniformBlockFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceInlineUniformBlockFeatures) -> VkPhysicalDeviceInlineUniformBlockFeatures {
        VkPhysicalDeviceInlineUniformBlockFeatures {
            inline_uniform_block: u32::vk_to_wrapped(&src.inline_uniform_block),
            descriptor_binding_inline_uniform_block_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_inline_uniform_block_update_after_bind),
        }
    }
}

impl Default for VkPhysicalDeviceInlineUniformBlockFeatures {
    fn default() -> VkPhysicalDeviceInlineUniformBlockFeatures {
        VkPhysicalDeviceInlineUniformBlockFeatures {
            inline_uniform_block: false,
            descriptor_binding_inline_uniform_block_update_after_bind: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceInlineUniformBlockFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceInlineUniformBlockFeatures {
    fn vk_free(&mut self) {
        
    }
}