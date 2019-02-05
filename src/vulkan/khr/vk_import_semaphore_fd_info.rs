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
use vulkan::vk::{VkSemaphore,RawVkSemaphore};
use vulkan::vk::{VkSemaphoreImportFlags,RawVkSemaphoreImportFlags};
use vulkan::vk::{VkExternalSemaphoreHandleTypeFlags,RawVkExternalSemaphoreHandleTypeFlags};

/// Wrapper for [VkImportSemaphoreFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImportSemaphoreFdInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkImportSemaphoreFdInfo {
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handle_type: VkExternalSemaphoreHandleTypeFlags,
    pub fd: i32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImportSemaphoreFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub semaphore: RawVkSemaphore,
    pub flags: RawVkSemaphoreImportFlags,
    pub handle_type: RawVkExternalSemaphoreHandleTypeFlags,
    pub fd: i32,
}

impl VkWrappedType<RawVkImportSemaphoreFdInfo> for VkImportSemaphoreFdInfo {
    fn vk_to_raw(src: &VkImportSemaphoreFdInfo, dst: &mut RawVkImportSemaphoreFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImportSemaphoreFdInfoKhr);
        dst.next = ptr::null_mut();
        dst.semaphore = vk_to_raw_value(&src.semaphore);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
        dst.fd = src.fd;
    }
}

impl VkRawType<VkImportSemaphoreFdInfo> for RawVkImportSemaphoreFdInfo {
    fn vk_to_wrapped(src: &RawVkImportSemaphoreFdInfo) -> VkImportSemaphoreFdInfo {
        VkImportSemaphoreFdInfo {
            semaphore: RawVkSemaphore::vk_to_wrapped(&src.semaphore),
            flags: RawVkSemaphoreImportFlags::vk_to_wrapped(&src.flags),
            handle_type: RawVkExternalSemaphoreHandleTypeFlags::vk_to_wrapped(&src.handle_type),
            fd: src.fd,
        }
    }
}

impl Default for VkImportSemaphoreFdInfo {
    fn default() -> VkImportSemaphoreFdInfo {
        VkImportSemaphoreFdInfo {
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: 0,
        }
    }
}

impl VkSetup for VkImportSemaphoreFdInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.semaphore, fn_table);
    }
}

impl VkFree for RawVkImportSemaphoreFdInfo {
    fn vk_free(&self) {
        
    }
}