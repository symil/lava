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

/// Wrapper for [VkImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkImageDrmFormatModifierProperties {
    pub drm_format_modifier: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageDrmFormatModifierProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub drm_format_modifier: u64,
}

impl VkWrappedType<RawVkImageDrmFormatModifierProperties> for VkImageDrmFormatModifierProperties {
    fn vk_to_raw(src: &VkImageDrmFormatModifierProperties, dst: &mut RawVkImageDrmFormatModifierProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageDrmFormatModifierPropertiesExt);
        dst.next = ptr::null_mut();
        dst.drm_format_modifier = vk_to_raw_value(&src.drm_format_modifier);
    }
}

impl VkRawType<VkImageDrmFormatModifierProperties> for RawVkImageDrmFormatModifierProperties {
    fn vk_to_wrapped(src: &RawVkImageDrmFormatModifierProperties) -> VkImageDrmFormatModifierProperties {
        VkImageDrmFormatModifierProperties {
            drm_format_modifier: u64::vk_to_wrapped(&src.drm_format_modifier),
        }
    }
}

impl Default for VkImageDrmFormatModifierProperties {
    fn default() -> VkImageDrmFormatModifierProperties {
        VkImageDrmFormatModifierProperties {
            drm_format_modifier: 0,
        }
    }
}

impl VkSetup for VkImageDrmFormatModifierProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkImageDrmFormatModifierProperties {
    fn vk_free(&self) {
        
    }
}