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

/// Wrapper for [VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDiscardRectangleProperties {
    pub max_discard_rectangles: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDiscardRectangleProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub max_discard_rectangles: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDiscardRectangleProperties> for VkPhysicalDeviceDiscardRectangleProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceDiscardRectangleProperties, dst: &mut RawVkPhysicalDeviceDiscardRectangleProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDiscardRectanglePropertiesExt);
        dst.next = ptr::null_mut();
        dst.max_discard_rectangles = vk_to_raw_value(&src.max_discard_rectangles);
    }
}

impl VkRawType<VkPhysicalDeviceDiscardRectangleProperties> for RawVkPhysicalDeviceDiscardRectangleProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDiscardRectangleProperties) -> VkPhysicalDeviceDiscardRectangleProperties {
        VkPhysicalDeviceDiscardRectangleProperties {
            max_discard_rectangles: u32::vk_to_wrapped(&src.max_discard_rectangles),
        }
    }
}

impl Default for VkPhysicalDeviceDiscardRectangleProperties {
    fn default() -> VkPhysicalDeviceDiscardRectangleProperties {
        VkPhysicalDeviceDiscardRectangleProperties {
            max_discard_rectangles: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDiscardRectangleProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDiscardRectangleProperties {
    fn vk_free(&self) {
        
    }
}