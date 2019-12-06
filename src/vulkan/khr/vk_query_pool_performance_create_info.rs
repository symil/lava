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

/// Wrapper for [VkQueryPoolPerformanceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkQueryPoolPerformanceCreateInfo {
    pub queue_family_index: usize,
    pub counter_indices: Vec<usize>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkQueryPoolPerformanceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub counter_indices: *mut u32,
}

impl VkWrappedType<RawVkQueryPoolPerformanceCreateInfo> for VkQueryPoolPerformanceCreateInfo {
    fn vk_to_raw(src: &VkQueryPoolPerformanceCreateInfo, dst: &mut RawVkQueryPoolPerformanceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::QueryPoolPerformanceCreateInfoKhr);
        dst.next = ptr::null_mut();
        dst.queue_family_index = vk_to_raw_value(&src.queue_family_index);
        dst.counter_index_count = src.counter_indices.len() as u32;
        dst.counter_indices = new_ptr_vk_array(&src.counter_indices);
    }
}

impl VkRawType<VkQueryPoolPerformanceCreateInfo> for RawVkQueryPoolPerformanceCreateInfo {
    fn vk_to_wrapped(src: &RawVkQueryPoolPerformanceCreateInfo) -> VkQueryPoolPerformanceCreateInfo {
        VkQueryPoolPerformanceCreateInfo {
            queue_family_index: u32::vk_to_wrapped(&src.queue_family_index),
            counter_indices: new_vk_array(src.counter_index_count, src.counter_indices),
        }
    }
}

impl Default for VkQueryPoolPerformanceCreateInfo {
    fn default() -> VkQueryPoolPerformanceCreateInfo {
        VkQueryPoolPerformanceCreateInfo {
            queue_family_index: 0,
            counter_indices: Vec::new(),
        }
    }
}

impl VkSetup for VkQueryPoolPerformanceCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkQueryPoolPerformanceCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.counter_indices);
    }
}