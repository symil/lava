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

/// Wrapper for [VkPhysicalDevicePushDescriptorPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDevicePushDescriptorProperties {
    pub max_push_descriptors: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDevicePushDescriptorProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub max_push_descriptors: u32,
}

impl VkWrappedType<RawVkPhysicalDevicePushDescriptorProperties> for VkPhysicalDevicePushDescriptorProperties {
    fn vk_to_raw(src: &VkPhysicalDevicePushDescriptorProperties, dst: &mut RawVkPhysicalDevicePushDescriptorProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDevicePushDescriptorPropertiesKhr);
        dst.next = ptr::null_mut();
        dst.max_push_descriptors = vk_to_raw_value(&src.max_push_descriptors);
    }
}

impl VkRawType<VkPhysicalDevicePushDescriptorProperties> for RawVkPhysicalDevicePushDescriptorProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDevicePushDescriptorProperties) -> VkPhysicalDevicePushDescriptorProperties {
        VkPhysicalDevicePushDescriptorProperties {
            max_push_descriptors: u32::vk_to_wrapped(&src.max_push_descriptors),
        }
    }
}

impl Default for VkPhysicalDevicePushDescriptorProperties {
    fn default() -> VkPhysicalDevicePushDescriptorProperties {
        VkPhysicalDevicePushDescriptorProperties {
            max_push_descriptors: 0,
        }
    }
}

impl VkSetup for VkPhysicalDevicePushDescriptorProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDevicePushDescriptorProperties {
    fn vk_free(&self) {
        
    }
}