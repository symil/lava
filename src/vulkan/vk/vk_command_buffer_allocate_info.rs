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
use vulkan::vk::{VkCommandPool,RawVkCommandPool};
use vulkan::vk::{VkCommandBufferLevel,RawVkCommandBufferLevel};

/// Wrapper for [VkCommandBufferAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferAllocateInfo.html).
#[derive(Debug, Clone)]
pub struct VkCommandBufferAllocateInfo {
    pub command_pool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub command_buffer_count: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandBufferAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub command_pool: RawVkCommandPool,
    pub level: RawVkCommandBufferLevel,
    pub command_buffer_count: u32,
}

impl VkWrappedType<RawVkCommandBufferAllocateInfo> for VkCommandBufferAllocateInfo {
    fn vk_to_raw(src: &VkCommandBufferAllocateInfo, dst: &mut RawVkCommandBufferAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandBufferAllocateInfo);
        dst.next = ptr::null_mut();
        dst.command_pool = vk_to_raw_value(&src.command_pool);
        dst.level = vk_to_raw_value(&src.level);
        dst.command_buffer_count = vk_to_raw_value(&src.command_buffer_count);
    }
}

impl VkRawType<VkCommandBufferAllocateInfo> for RawVkCommandBufferAllocateInfo {
    fn vk_to_wrapped(src: &RawVkCommandBufferAllocateInfo) -> VkCommandBufferAllocateInfo {
        VkCommandBufferAllocateInfo {
            command_pool: RawVkCommandPool::vk_to_wrapped(&src.command_pool),
            level: RawVkCommandBufferLevel::vk_to_wrapped(&src.level),
            command_buffer_count: u32::vk_to_wrapped(&src.command_buffer_count),
        }
    }
}

impl Default for VkCommandBufferAllocateInfo {
    fn default() -> VkCommandBufferAllocateInfo {
        VkCommandBufferAllocateInfo {
            command_pool: Default::default(),
            level: Default::default(),
            command_buffer_count: 0,
        }
    }
}

impl VkSetup for VkCommandBufferAllocateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.command_pool, fn_table);
    }
}

impl VkFree for RawVkCommandBufferAllocateInfo {
    fn vk_free(&self) {
        
    }
}