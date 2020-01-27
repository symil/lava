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

/// Wrapper for [VkVertexInputBindingDivisorDescriptionEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html).
#[derive(Debug, Clone)]
pub struct VkVertexInputBindingDivisorDescription {
    pub binding: usize,
    pub divisor: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkVertexInputBindingDivisorDescription {
    pub binding: u32,
    pub divisor: u32,
}

impl VkWrappedType<RawVkVertexInputBindingDivisorDescription> for VkVertexInputBindingDivisorDescription {
    fn vk_to_raw(src: &VkVertexInputBindingDivisorDescription, dst: &mut RawVkVertexInputBindingDivisorDescription) {
        dst.binding = vk_to_raw_value(&src.binding);
        dst.divisor = vk_to_raw_value(&src.divisor);
    }
}

impl VkRawType<VkVertexInputBindingDivisorDescription> for RawVkVertexInputBindingDivisorDescription {
    fn vk_to_wrapped(src: &RawVkVertexInputBindingDivisorDescription) -> VkVertexInputBindingDivisorDescription {
        VkVertexInputBindingDivisorDescription {
            binding: u32::vk_to_wrapped(&src.binding),
            divisor: u32::vk_to_wrapped(&src.divisor),
        }
    }
}

impl Default for VkVertexInputBindingDivisorDescription {
    fn default() -> VkVertexInputBindingDivisorDescription {
        VkVertexInputBindingDivisorDescription {
            binding: 0,
            divisor: 0,
        }
    }
}

impl VkSetup for VkVertexInputBindingDivisorDescription {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkVertexInputBindingDivisorDescription {
    fn vk_free(&self) {
        
    }
}