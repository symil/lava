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

/// Wrapper for [VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceConditionalRenderingFeatures {
    pub conditional_rendering: bool,
    pub inherited_conditional_rendering: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceConditionalRenderingFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub conditional_rendering: u32,
    pub inherited_conditional_rendering: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceConditionalRenderingFeatures> for VkPhysicalDeviceConditionalRenderingFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceConditionalRenderingFeatures, dst: &mut RawVkPhysicalDeviceConditionalRenderingFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceConditionalRenderingFeaturesExt);
        dst.next = ptr::null();
        dst.conditional_rendering = vk_to_raw_value(&src.conditional_rendering);
        dst.inherited_conditional_rendering = vk_to_raw_value(&src.inherited_conditional_rendering);
    }
}

impl VkRawType<VkPhysicalDeviceConditionalRenderingFeatures> for RawVkPhysicalDeviceConditionalRenderingFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceConditionalRenderingFeatures) -> VkPhysicalDeviceConditionalRenderingFeatures {
        VkPhysicalDeviceConditionalRenderingFeatures {
            conditional_rendering: u32::vk_to_wrapped(&src.conditional_rendering),
            inherited_conditional_rendering: u32::vk_to_wrapped(&src.inherited_conditional_rendering),
        }
    }
}

impl Default for VkPhysicalDeviceConditionalRenderingFeatures {
    fn default() -> VkPhysicalDeviceConditionalRenderingFeatures {
        VkPhysicalDeviceConditionalRenderingFeatures {
            conditional_rendering: false,
            inherited_conditional_rendering: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceConditionalRenderingFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceConditionalRenderingFeatures {
    fn vk_free(&mut self) {
        
    }
}