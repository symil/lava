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

/// Wrapper for [VkMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryFdPropertiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkMemoryFdProperties {
    pub memory_type_bits: u32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryFdProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub memory_type_bits: u32,
}

impl VkWrappedType<RawVkMemoryFdProperties> for VkMemoryFdProperties {
    fn vk_to_raw(src: &VkMemoryFdProperties, dst: &mut RawVkMemoryFdProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryFdPropertiesKhr);
        dst.next = ptr::null();
        dst.memory_type_bits = src.memory_type_bits;
    }
}

impl VkRawType<VkMemoryFdProperties> for RawVkMemoryFdProperties {
    fn vk_to_wrapped(src: &RawVkMemoryFdProperties) -> VkMemoryFdProperties {
        VkMemoryFdProperties {
            memory_type_bits: src.memory_type_bits,
        }
    }
}

impl Default for VkMemoryFdProperties {
    fn default() -> VkMemoryFdProperties {
        VkMemoryFdProperties {
            memory_type_bits: 0,
        }
    }
}

impl VkSetup for VkMemoryFdProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkMemoryFdProperties {
    fn vk_free(&mut self) {
        
    }
}