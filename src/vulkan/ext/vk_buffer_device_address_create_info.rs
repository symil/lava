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

/// Wrapper for [VkBufferDeviceAddressCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkBufferDeviceAddressCreateInfo {
    pub device_address: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferDeviceAddressCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub device_address: u64,
}

impl VkWrappedType<RawVkBufferDeviceAddressCreateInfo> for VkBufferDeviceAddressCreateInfo {
    fn vk_to_raw(src: &VkBufferDeviceAddressCreateInfo, dst: &mut RawVkBufferDeviceAddressCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferDeviceAddressCreateInfoExt);
        dst.next = ptr::null();
        dst.device_address = vk_to_raw_value(&src.device_address);
    }
}

impl VkRawType<VkBufferDeviceAddressCreateInfo> for RawVkBufferDeviceAddressCreateInfo {
    fn vk_to_wrapped(src: &RawVkBufferDeviceAddressCreateInfo) -> VkBufferDeviceAddressCreateInfo {
        VkBufferDeviceAddressCreateInfo {
            device_address: u64::vk_to_wrapped(&src.device_address),
        }
    }
}

impl Default for VkBufferDeviceAddressCreateInfo {
    fn default() -> VkBufferDeviceAddressCreateInfo {
        VkBufferDeviceAddressCreateInfo {
            device_address: 0,
        }
    }
}

impl VkSetup for VkBufferDeviceAddressCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkBufferDeviceAddressCreateInfo {
    fn vk_free(&mut self) {
        
    }
}