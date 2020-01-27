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
use vulkan::vk::{VkExternalMemoryHandleTypeFlags,RawVkExternalMemoryHandleTypeFlags};

/// Wrapper for [VkExternalMemoryImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkExternalMemoryImageCreateInfo {
    pub handle_types: VkExternalMemoryHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExternalMemoryImageCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub handle_types: RawVkExternalMemoryHandleTypeFlags,
}

impl VkWrappedType<RawVkExternalMemoryImageCreateInfo> for VkExternalMemoryImageCreateInfo {
    fn vk_to_raw(src: &VkExternalMemoryImageCreateInfo, dst: &mut RawVkExternalMemoryImageCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExternalMemoryImageCreateInfo);
        dst.next = ptr::null_mut();
        dst.handle_types = vk_to_raw_value(&src.handle_types);
    }
}

impl VkRawType<VkExternalMemoryImageCreateInfo> for RawVkExternalMemoryImageCreateInfo {
    fn vk_to_wrapped(src: &RawVkExternalMemoryImageCreateInfo) -> VkExternalMemoryImageCreateInfo {
        VkExternalMemoryImageCreateInfo {
            handle_types: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.handle_types),
        }
    }
}

impl Default for VkExternalMemoryImageCreateInfo {
    fn default() -> VkExternalMemoryImageCreateInfo {
        VkExternalMemoryImageCreateInfo {
            handle_types: Default::default(),
        }
    }
}

impl VkSetup for VkExternalMemoryImageCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkExternalMemoryImageCreateInfo {
    fn vk_free(&self) {
        
    }
}