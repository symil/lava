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
use vulkan::vk::{VkExternalSemaphoreHandleTypeFlags,RawVkExternalSemaphoreHandleTypeFlags};

/// Wrapper for [VkExportSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkExportSemaphoreCreateInfo {
    pub handle_types: VkExternalSemaphoreHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExportSemaphoreCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub handle_types: RawVkExternalSemaphoreHandleTypeFlags,
}

impl VkWrappedType<RawVkExportSemaphoreCreateInfo> for VkExportSemaphoreCreateInfo {
    fn vk_to_raw(src: &VkExportSemaphoreCreateInfo, dst: &mut RawVkExportSemaphoreCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExportSemaphoreCreateInfo);
        dst.next = ptr::null_mut();
        dst.handle_types = vk_to_raw_value(&src.handle_types);
    }
}

impl VkRawType<VkExportSemaphoreCreateInfo> for RawVkExportSemaphoreCreateInfo {
    fn vk_to_wrapped(src: &RawVkExportSemaphoreCreateInfo) -> VkExportSemaphoreCreateInfo {
        VkExportSemaphoreCreateInfo {
            handle_types: RawVkExternalSemaphoreHandleTypeFlags::vk_to_wrapped(&src.handle_types),
        }
    }
}

impl Default for VkExportSemaphoreCreateInfo {
    fn default() -> VkExportSemaphoreCreateInfo {
        VkExportSemaphoreCreateInfo {
            handle_types: Default::default(),
        }
    }
}

impl VkSetup for VkExportSemaphoreCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkExportSemaphoreCreateInfo {
    fn vk_free(&self) {
        
    }
}