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

/// Wrapper for [VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceIndexTypeUint8Features {
    pub index_type_uint_8: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceIndexTypeUint8Features {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub index_type_uint_8: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceIndexTypeUint8Features> for VkPhysicalDeviceIndexTypeUint8Features {
    fn vk_to_raw(src: &VkPhysicalDeviceIndexTypeUint8Features, dst: &mut RawVkPhysicalDeviceIndexTypeUint8Features) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceIndexTypeUint8FeaturesExt);
        dst.next = ptr::null_mut();
        dst.index_type_uint_8 = vk_to_raw_value(&src.index_type_uint_8);
    }
}

impl VkRawType<VkPhysicalDeviceIndexTypeUint8Features> for RawVkPhysicalDeviceIndexTypeUint8Features {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceIndexTypeUint8Features) -> VkPhysicalDeviceIndexTypeUint8Features {
        VkPhysicalDeviceIndexTypeUint8Features {
            index_type_uint_8: u32::vk_to_wrapped(&src.index_type_uint_8),
        }
    }
}

impl Default for VkPhysicalDeviceIndexTypeUint8Features {
    fn default() -> VkPhysicalDeviceIndexTypeUint8Features {
        VkPhysicalDeviceIndexTypeUint8Features {
            index_type_uint_8: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceIndexTypeUint8Features {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceIndexTypeUint8Features {
    fn vk_free(&self) {
        
    }
}