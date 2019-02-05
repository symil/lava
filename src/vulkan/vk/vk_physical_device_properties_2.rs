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
use vulkan::vk::{VkPhysicalDeviceProperties,RawVkPhysicalDeviceProperties};

/// Wrapper for [VkPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceProperties2.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProperties2 {
    pub properties: VkPhysicalDeviceProperties,
}

#[doc(hidden)]
#[repr(C)]
pub struct RawVkPhysicalDeviceProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub properties: RawVkPhysicalDeviceProperties,
}

impl VkRawType<VkPhysicalDeviceProperties2> for RawVkPhysicalDeviceProperties2 {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceProperties2) -> VkPhysicalDeviceProperties2 {
        VkPhysicalDeviceProperties2 {
            properties: RawVkPhysicalDeviceProperties::vk_to_wrapped(&src.properties),
        }
    }
}

impl VkSetup for VkPhysicalDeviceProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.properties, fn_table);
    }
}

impl VkFree for RawVkPhysicalDeviceProperties2 {
    fn vk_free(&mut self) {
        RawVkPhysicalDeviceProperties::vk_free(&mut self.properties);
    }
}