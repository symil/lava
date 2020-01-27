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
use vulkan::vk::{VkExternalFenceHandleTypeFlags,RawVkExternalFenceHandleTypeFlags};

/// Wrapper for [VkPhysicalDeviceExternalFenceInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceExternalFenceInfo {
    pub handle_type: VkExternalFenceHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceExternalFenceInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub handle_type: RawVkExternalFenceHandleTypeFlags,
}

impl VkWrappedType<RawVkPhysicalDeviceExternalFenceInfo> for VkPhysicalDeviceExternalFenceInfo {
    fn vk_to_raw(src: &VkPhysicalDeviceExternalFenceInfo, dst: &mut RawVkPhysicalDeviceExternalFenceInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceExternalFenceInfo);
        dst.next = ptr::null_mut();
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl VkRawType<VkPhysicalDeviceExternalFenceInfo> for RawVkPhysicalDeviceExternalFenceInfo {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceExternalFenceInfo) -> VkPhysicalDeviceExternalFenceInfo {
        VkPhysicalDeviceExternalFenceInfo {
            handle_type: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.handle_type),
        }
    }
}

impl Default for VkPhysicalDeviceExternalFenceInfo {
    fn default() -> VkPhysicalDeviceExternalFenceInfo {
        VkPhysicalDeviceExternalFenceInfo {
            handle_type: Default::default(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceExternalFenceInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceExternalFenceInfo {
    fn vk_free(&self) {
        
    }
}