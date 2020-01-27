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
use vulkan::ext::{VkDrmFormatModifierProperties,RawVkDrmFormatModifierProperties};

/// Wrapper for [VkDrmFormatModifierPropertiesListEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html).
#[derive(Debug, Clone)]
pub struct VkDrmFormatModifierPropertiesList {
    pub drm_format_modifier_properties: Option<Vec<VkDrmFormatModifierProperties>>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDrmFormatModifierPropertiesList {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub drm_format_modifier_properties: *mut RawVkDrmFormatModifierProperties,
}

impl VkWrappedType<RawVkDrmFormatModifierPropertiesList> for VkDrmFormatModifierPropertiesList {
    fn vk_to_raw(src: &VkDrmFormatModifierPropertiesList, dst: &mut RawVkDrmFormatModifierPropertiesList) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DrmFormatModifierPropertiesListExt);
        dst.next = ptr::null_mut();
        dst.drm_format_modifier_count = get_array_option_len(&src.drm_format_modifier_properties) as u32;
        dst.drm_format_modifier_properties = new_ptr_vk_array_checked(&src.drm_format_modifier_properties);
    }
}

impl VkRawType<VkDrmFormatModifierPropertiesList> for RawVkDrmFormatModifierPropertiesList {
    fn vk_to_wrapped(src: &RawVkDrmFormatModifierPropertiesList) -> VkDrmFormatModifierPropertiesList {
        VkDrmFormatModifierPropertiesList {
            drm_format_modifier_properties: new_vk_array_checked(src.drm_format_modifier_count, src.drm_format_modifier_properties),
        }
    }
}

impl Default for VkDrmFormatModifierPropertiesList {
    fn default() -> VkDrmFormatModifierPropertiesList {
        VkDrmFormatModifierPropertiesList {
            drm_format_modifier_properties: None,
        }
    }
}

impl VkSetup for VkDrmFormatModifierPropertiesList {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDrmFormatModifierPropertiesList {
    fn vk_free(&self) {
        free_vk_ptr_array(self.drm_format_modifier_count as usize, self.drm_format_modifier_properties);
    }
}