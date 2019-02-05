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

/// Wrapper for [VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeatures {
    pub vertex_attribute_instance_rate_divisor: bool,
    pub vertex_attribute_instance_rate_zero_divisor: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceVertexAttributeDivisorFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub vertex_attribute_instance_rate_divisor: u32,
    pub vertex_attribute_instance_rate_zero_divisor: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceVertexAttributeDivisorFeatures> for VkPhysicalDeviceVertexAttributeDivisorFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceVertexAttributeDivisorFeatures, dst: &mut RawVkPhysicalDeviceVertexAttributeDivisorFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceVertexAttributeDivisorFeaturesExt);
        dst.next = ptr::null();
        dst.vertex_attribute_instance_rate_divisor = vk_to_raw_value(&src.vertex_attribute_instance_rate_divisor);
        dst.vertex_attribute_instance_rate_zero_divisor = vk_to_raw_value(&src.vertex_attribute_instance_rate_zero_divisor);
    }
}

impl VkRawType<VkPhysicalDeviceVertexAttributeDivisorFeatures> for RawVkPhysicalDeviceVertexAttributeDivisorFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceVertexAttributeDivisorFeatures) -> VkPhysicalDeviceVertexAttributeDivisorFeatures {
        VkPhysicalDeviceVertexAttributeDivisorFeatures {
            vertex_attribute_instance_rate_divisor: u32::vk_to_wrapped(&src.vertex_attribute_instance_rate_divisor),
            vertex_attribute_instance_rate_zero_divisor: u32::vk_to_wrapped(&src.vertex_attribute_instance_rate_zero_divisor),
        }
    }
}

impl Default for VkPhysicalDeviceVertexAttributeDivisorFeatures {
    fn default() -> VkPhysicalDeviceVertexAttributeDivisorFeatures {
        VkPhysicalDeviceVertexAttributeDivisorFeatures {
            vertex_attribute_instance_rate_divisor: false,
            vertex_attribute_instance_rate_zero_divisor: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceVertexAttributeDivisorFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceVertexAttributeDivisorFeatures {
    fn vk_free(&mut self) {
        
    }
}