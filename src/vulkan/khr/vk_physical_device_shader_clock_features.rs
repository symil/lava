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

/// Wrapper for [VkPhysicalDeviceShaderClockFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceShaderClockFeatures {
    pub shader_subgroup_clock: bool,
    pub shader_device_clock: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceShaderClockFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub shader_subgroup_clock: u32,
    pub shader_device_clock: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceShaderClockFeatures> for VkPhysicalDeviceShaderClockFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceShaderClockFeatures, dst: &mut RawVkPhysicalDeviceShaderClockFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceShaderClockFeaturesKhr);
        dst.next = ptr::null_mut();
        dst.shader_subgroup_clock = vk_to_raw_value(&src.shader_subgroup_clock);
        dst.shader_device_clock = vk_to_raw_value(&src.shader_device_clock);
    }
}

impl VkRawType<VkPhysicalDeviceShaderClockFeatures> for RawVkPhysicalDeviceShaderClockFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceShaderClockFeatures) -> VkPhysicalDeviceShaderClockFeatures {
        VkPhysicalDeviceShaderClockFeatures {
            shader_subgroup_clock: u32::vk_to_wrapped(&src.shader_subgroup_clock),
            shader_device_clock: u32::vk_to_wrapped(&src.shader_device_clock),
        }
    }
}

impl Default for VkPhysicalDeviceShaderClockFeatures {
    fn default() -> VkPhysicalDeviceShaderClockFeatures {
        VkPhysicalDeviceShaderClockFeatures {
            shader_subgroup_clock: false,
            shader_device_clock: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceShaderClockFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceShaderClockFeatures {
    fn vk_free(&self) {
        
    }
}