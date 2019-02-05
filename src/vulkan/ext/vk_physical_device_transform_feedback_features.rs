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

/// Wrapper for [VkPhysicalDeviceTransformFeedbackFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceTransformFeedbackFeatures {
    pub transform_feedback: bool,
    pub geometry_streams: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceTransformFeedbackFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub transform_feedback: u32,
    pub geometry_streams: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceTransformFeedbackFeatures> for VkPhysicalDeviceTransformFeedbackFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceTransformFeedbackFeatures, dst: &mut RawVkPhysicalDeviceTransformFeedbackFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceTransformFeedbackFeaturesExt);
        dst.next = ptr::null_mut();
        dst.transform_feedback = vk_to_raw_value(&src.transform_feedback);
        dst.geometry_streams = vk_to_raw_value(&src.geometry_streams);
    }
}

impl VkRawType<VkPhysicalDeviceTransformFeedbackFeatures> for RawVkPhysicalDeviceTransformFeedbackFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceTransformFeedbackFeatures) -> VkPhysicalDeviceTransformFeedbackFeatures {
        VkPhysicalDeviceTransformFeedbackFeatures {
            transform_feedback: u32::vk_to_wrapped(&src.transform_feedback),
            geometry_streams: u32::vk_to_wrapped(&src.geometry_streams),
        }
    }
}

impl Default for VkPhysicalDeviceTransformFeedbackFeatures {
    fn default() -> VkPhysicalDeviceTransformFeedbackFeatures {
        VkPhysicalDeviceTransformFeedbackFeatures {
            transform_feedback: false,
            geometry_streams: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceTransformFeedbackFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceTransformFeedbackFeatures {
    fn vk_free(&self) {
        
    }
}