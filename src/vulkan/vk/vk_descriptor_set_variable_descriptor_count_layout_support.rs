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

/// Wrapper for [VkDescriptorSetVariableDescriptorCountLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html).
#[derive(Debug, Clone)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
    pub max_variable_descriptor_count: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub max_variable_descriptor_count: u32,
}

impl VkWrappedType<RawVkDescriptorSetVariableDescriptorCountLayoutSupport> for VkDescriptorSetVariableDescriptorCountLayoutSupport {
    fn vk_to_raw(src: &VkDescriptorSetVariableDescriptorCountLayoutSupport, dst: &mut RawVkDescriptorSetVariableDescriptorCountLayoutSupport) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorSetVariableDescriptorCountLayoutSupport);
        dst.next = ptr::null_mut();
        dst.max_variable_descriptor_count = vk_to_raw_value(&src.max_variable_descriptor_count);
    }
}

impl VkRawType<VkDescriptorSetVariableDescriptorCountLayoutSupport> for RawVkDescriptorSetVariableDescriptorCountLayoutSupport {
    fn vk_to_wrapped(src: &RawVkDescriptorSetVariableDescriptorCountLayoutSupport) -> VkDescriptorSetVariableDescriptorCountLayoutSupport {
        VkDescriptorSetVariableDescriptorCountLayoutSupport {
            max_variable_descriptor_count: u32::vk_to_wrapped(&src.max_variable_descriptor_count),
        }
    }
}

impl Default for VkDescriptorSetVariableDescriptorCountLayoutSupport {
    fn default() -> VkDescriptorSetVariableDescriptorCountLayoutSupport {
        VkDescriptorSetVariableDescriptorCountLayoutSupport {
            max_variable_descriptor_count: 0,
        }
    }
}

impl VkSetup for VkDescriptorSetVariableDescriptorCountLayoutSupport {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDescriptorSetVariableDescriptorCountLayoutSupport {
    fn vk_free(&self) {
        
    }
}