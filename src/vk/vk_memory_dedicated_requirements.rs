// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryDedicatedRequirements {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub prefers_dedicated_allocation: u32,
    pub requires_dedicated_allocation: u32,
}

#[derive(Debug, Clone)]
pub struct VkMemoryDedicatedRequirements {
    pub prefers_dedicated_allocation: bool,
    pub requires_dedicated_allocation: bool,
}

impl VkRawType<VkMemoryDedicatedRequirements> for RawVkMemoryDedicatedRequirements {
    fn vk_to_wrapped(src: &RawVkMemoryDedicatedRequirements) -> VkMemoryDedicatedRequirements {
        VkMemoryDedicatedRequirements {
            prefers_dedicated_allocation: u32::vk_to_wrapped(&src.prefers_dedicated_allocation),
            requires_dedicated_allocation: u32::vk_to_wrapped(&src.requires_dedicated_allocation),
        }
    }
}

impl VkWrappedType<RawVkMemoryDedicatedRequirements> for VkMemoryDedicatedRequirements {
    fn vk_to_raw(src: &VkMemoryDedicatedRequirements, dst: &mut RawVkMemoryDedicatedRequirements) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryDedicatedRequirements);
        dst.next = ptr::null();
        dst.prefers_dedicated_allocation = vk_to_raw_value(&src.prefers_dedicated_allocation);
        dst.requires_dedicated_allocation = vk_to_raw_value(&src.requires_dedicated_allocation);
    }
}

impl Default for VkMemoryDedicatedRequirements {
    fn default() -> VkMemoryDedicatedRequirements {
        VkMemoryDedicatedRequirements {
            prefers_dedicated_allocation: false,
            requires_dedicated_allocation: false,
        }
    }
}

impl VkSetup for VkMemoryDedicatedRequirements {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMemoryDedicatedRequirements {
    fn vk_free(&mut self) {
        
    }
}