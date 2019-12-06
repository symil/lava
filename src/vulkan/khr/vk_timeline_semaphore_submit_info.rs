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

/// Wrapper for [VkTimelineSemaphoreSubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkTimelineSemaphoreSubmitInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkTimelineSemaphoreSubmitInfo {
    pub wait_semaphore_values: Option<Vec<usize>>,
    pub signal_semaphore_values: Option<Vec<usize>>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkTimelineSemaphoreSubmitInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub wait_semaphore_value_count: u32,
    pub wait_semaphore_values: *mut u64,
    pub signal_semaphore_value_count: u32,
    pub signal_semaphore_values: *mut u64,
}

impl VkWrappedType<RawVkTimelineSemaphoreSubmitInfo> for VkTimelineSemaphoreSubmitInfo {
    fn vk_to_raw(src: &VkTimelineSemaphoreSubmitInfo, dst: &mut RawVkTimelineSemaphoreSubmitInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::TimelineSemaphoreSubmitInfoKhr);
        dst.next = ptr::null_mut();
        dst.wait_semaphore_value_count = get_array_option_len(&src.wait_semaphore_values) as u32;
        dst.wait_semaphore_values = new_ptr_vk_array_checked(&src.wait_semaphore_values);
        dst.signal_semaphore_value_count = get_array_option_len(&src.signal_semaphore_values) as u32;
        dst.signal_semaphore_values = new_ptr_vk_array_checked(&src.signal_semaphore_values);
    }
}

impl VkRawType<VkTimelineSemaphoreSubmitInfo> for RawVkTimelineSemaphoreSubmitInfo {
    fn vk_to_wrapped(src: &RawVkTimelineSemaphoreSubmitInfo) -> VkTimelineSemaphoreSubmitInfo {
        VkTimelineSemaphoreSubmitInfo {
            wait_semaphore_values: new_vk_array_checked(src.wait_semaphore_value_count, src.wait_semaphore_values),
            signal_semaphore_values: new_vk_array_checked(src.signal_semaphore_value_count, src.signal_semaphore_values),
        }
    }
}

impl Default for VkTimelineSemaphoreSubmitInfo {
    fn default() -> VkTimelineSemaphoreSubmitInfo {
        VkTimelineSemaphoreSubmitInfo {
            wait_semaphore_values: None,
            signal_semaphore_values: None,
        }
    }
}

impl VkSetup for VkTimelineSemaphoreSubmitInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkTimelineSemaphoreSubmitInfo {
    fn vk_free(&self) {
        free_ptr(self.wait_semaphore_values);
        free_ptr(self.signal_semaphore_values);
    }
}