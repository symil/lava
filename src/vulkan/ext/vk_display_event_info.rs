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
use vulkan::ext::{VkDisplayEventType,RawVkDisplayEventType};

/// Wrapper for [VkDisplayEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayEventInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkDisplayEventInfo {
    pub display_event: VkDisplayEventType,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayEventInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub display_event: RawVkDisplayEventType,
}

impl VkWrappedType<RawVkDisplayEventInfo> for VkDisplayEventInfo {
    fn vk_to_raw(src: &VkDisplayEventInfo, dst: &mut RawVkDisplayEventInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayEventInfoExt);
        dst.next = ptr::null_mut();
        dst.display_event = vk_to_raw_value(&src.display_event);
    }
}

impl VkRawType<VkDisplayEventInfo> for RawVkDisplayEventInfo {
    fn vk_to_wrapped(src: &RawVkDisplayEventInfo) -> VkDisplayEventInfo {
        VkDisplayEventInfo {
            display_event: RawVkDisplayEventType::vk_to_wrapped(&src.display_event),
        }
    }
}

impl Default for VkDisplayEventInfo {
    fn default() -> VkDisplayEventInfo {
        VkDisplayEventInfo {
            display_event: Default::default(),
        }
    }
}

impl VkSetup for VkDisplayEventInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDisplayEventInfo {
    fn vk_free(&self) {
        
    }
}