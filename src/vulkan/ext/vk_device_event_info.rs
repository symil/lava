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
use vulkan::ext::{VkDeviceEventType,RawVkDeviceEventType};

/// Wrapper for [VkDeviceEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkDeviceEventInfo {
    pub device_event: VkDeviceEventType,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceEventInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub device_event: RawVkDeviceEventType,
}

impl VkWrappedType<RawVkDeviceEventInfo> for VkDeviceEventInfo {
    fn vk_to_raw(src: &VkDeviceEventInfo, dst: &mut RawVkDeviceEventInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceEventInfoExt);
        dst.next = ptr::null_mut();
        dst.device_event = vk_to_raw_value(&src.device_event);
    }
}

impl VkRawType<VkDeviceEventInfo> for RawVkDeviceEventInfo {
    fn vk_to_wrapped(src: &RawVkDeviceEventInfo) -> VkDeviceEventInfo {
        VkDeviceEventInfo {
            device_event: RawVkDeviceEventType::vk_to_wrapped(&src.device_event),
        }
    }
}

impl Default for VkDeviceEventInfo {
    fn default() -> VkDeviceEventInfo {
        VkDeviceEventInfo {
            device_event: Default::default(),
        }
    }
}

impl VkSetup for VkDeviceEventInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDeviceEventInfo {
    fn vk_free(&self) {
        
    }
}