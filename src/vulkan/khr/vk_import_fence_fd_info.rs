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
use vulkan::vk::{VkFence,RawVkFence};
use vulkan::vk::{VkFenceImportFlags,RawVkFenceImportFlags};
use vulkan::vk::{VkExternalFenceHandleTypeFlags,RawVkExternalFenceHandleTypeFlags};

/// Wrapper for [VkImportFenceFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceFdInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkImportFenceFdInfo {
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handle_type: VkExternalFenceHandleTypeFlags,
    pub fd: i32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImportFenceFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub fence: RawVkFence,
    pub flags: RawVkFenceImportFlags,
    pub handle_type: RawVkExternalFenceHandleTypeFlags,
    pub fd: i32,
}

impl VkWrappedType<RawVkImportFenceFdInfo> for VkImportFenceFdInfo {
    fn vk_to_raw(src: &VkImportFenceFdInfo, dst: &mut RawVkImportFenceFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImportFenceFdInfoKhr);
        dst.next = ptr::null_mut();
        dst.fence = vk_to_raw_value(&src.fence);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
        dst.fd = src.fd;
    }
}

impl VkRawType<VkImportFenceFdInfo> for RawVkImportFenceFdInfo {
    fn vk_to_wrapped(src: &RawVkImportFenceFdInfo) -> VkImportFenceFdInfo {
        VkImportFenceFdInfo {
            fence: RawVkFence::vk_to_wrapped(&src.fence),
            flags: RawVkFenceImportFlags::vk_to_wrapped(&src.flags),
            handle_type: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.handle_type),
            fd: src.fd,
        }
    }
}

impl Default for VkImportFenceFdInfo {
    fn default() -> VkImportFenceFdInfo {
        VkImportFenceFdInfo {
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: 0,
        }
    }
}

impl VkSetup for VkImportFenceFdInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.fence, fn_table);
    }
}

impl VkFree for RawVkImportFenceFdInfo {
    fn vk_free(&self) {
        
    }
}