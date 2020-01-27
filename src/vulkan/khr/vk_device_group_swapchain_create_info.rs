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
use vulkan::khr::{VkDeviceGroupPresentModeFlags,RawVkDeviceGroupPresentModeFlags};

/// Wrapper for [VkDeviceGroupSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkDeviceGroupSwapchainCreateInfo {
    pub modes: VkDeviceGroupPresentModeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupSwapchainCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub modes: RawVkDeviceGroupPresentModeFlags,
}

impl VkWrappedType<RawVkDeviceGroupSwapchainCreateInfo> for VkDeviceGroupSwapchainCreateInfo {
    fn vk_to_raw(src: &VkDeviceGroupSwapchainCreateInfo, dst: &mut RawVkDeviceGroupSwapchainCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupSwapchainCreateInfoKhr);
        dst.next = ptr::null_mut();
        dst.modes = vk_to_raw_value(&src.modes);
    }
}

impl VkRawType<VkDeviceGroupSwapchainCreateInfo> for RawVkDeviceGroupSwapchainCreateInfo {
    fn vk_to_wrapped(src: &RawVkDeviceGroupSwapchainCreateInfo) -> VkDeviceGroupSwapchainCreateInfo {
        VkDeviceGroupSwapchainCreateInfo {
            modes: RawVkDeviceGroupPresentModeFlags::vk_to_wrapped(&src.modes),
        }
    }
}

impl Default for VkDeviceGroupSwapchainCreateInfo {
    fn default() -> VkDeviceGroupSwapchainCreateInfo {
        VkDeviceGroupSwapchainCreateInfo {
            modes: Default::default(),
        }
    }
}

impl VkSetup for VkDeviceGroupSwapchainCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDeviceGroupSwapchainCreateInfo {
    fn vk_free(&self) {
        
    }
}