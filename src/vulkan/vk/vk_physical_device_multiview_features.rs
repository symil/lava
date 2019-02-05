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

/// Wrapper for [VkPhysicalDeviceMultiviewFeatures](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMultiviewFeatures {
    pub multiview: bool,
    pub multiview_geometry_shader: bool,
    pub multiview_tessellation_shader: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMultiviewFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub multiview: u32,
    pub multiview_geometry_shader: u32,
    pub multiview_tessellation_shader: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceMultiviewFeatures> for VkPhysicalDeviceMultiviewFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceMultiviewFeatures, dst: &mut RawVkPhysicalDeviceMultiviewFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMultiviewFeatures);
        dst.next = ptr::null();
        dst.multiview = vk_to_raw_value(&src.multiview);
        dst.multiview_geometry_shader = vk_to_raw_value(&src.multiview_geometry_shader);
        dst.multiview_tessellation_shader = vk_to_raw_value(&src.multiview_tessellation_shader);
    }
}

impl VkRawType<VkPhysicalDeviceMultiviewFeatures> for RawVkPhysicalDeviceMultiviewFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMultiviewFeatures) -> VkPhysicalDeviceMultiviewFeatures {
        VkPhysicalDeviceMultiviewFeatures {
            multiview: u32::vk_to_wrapped(&src.multiview),
            multiview_geometry_shader: u32::vk_to_wrapped(&src.multiview_geometry_shader),
            multiview_tessellation_shader: u32::vk_to_wrapped(&src.multiview_tessellation_shader),
        }
    }
}

impl Default for VkPhysicalDeviceMultiviewFeatures {
    fn default() -> VkPhysicalDeviceMultiviewFeatures {
        VkPhysicalDeviceMultiviewFeatures {
            multiview: false,
            multiview_geometry_shader: false,
            multiview_tessellation_shader: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceMultiviewFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMultiviewFeatures {
    fn vk_free(&mut self) {
        
    }
}