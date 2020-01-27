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

/// Wrapper for [VkDispatchIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDispatchIndirectCommand.html).
#[derive(Debug, Clone)]
pub struct VkDispatchIndirectCommand {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl VkWrappedType<RawVkDispatchIndirectCommand> for VkDispatchIndirectCommand {
    fn vk_to_raw(src: &VkDispatchIndirectCommand, dst: &mut RawVkDispatchIndirectCommand) {
        dst.x = vk_to_raw_value(&src.x);
        dst.y = vk_to_raw_value(&src.y);
        dst.z = vk_to_raw_value(&src.z);
    }
}

impl VkRawType<VkDispatchIndirectCommand> for RawVkDispatchIndirectCommand {
    fn vk_to_wrapped(src: &RawVkDispatchIndirectCommand) -> VkDispatchIndirectCommand {
        VkDispatchIndirectCommand {
            x: u32::vk_to_wrapped(&src.x),
            y: u32::vk_to_wrapped(&src.y),
            z: u32::vk_to_wrapped(&src.z),
        }
    }
}

impl Default for VkDispatchIndirectCommand {
    fn default() -> VkDispatchIndirectCommand {
        VkDispatchIndirectCommand {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl VkSetup for VkDispatchIndirectCommand {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDispatchIndirectCommand {
    fn vk_free(&self) {
        
    }
}