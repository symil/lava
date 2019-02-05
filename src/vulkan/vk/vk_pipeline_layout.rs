// Generated by `scripts/generate.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vulkan::*;
use vulkan::vk::*;

#[doc(hidden)]
pub type RawVkPipelineLayout = u64;

/// Wrapper for [VkPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineLayout.html).
#[derive(Debug, Clone)]
pub struct VkPipelineLayout {
    _handle: RawVkPipelineLayout,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkPipelineLayout> for RawVkPipelineLayout {
    fn vk_to_wrapped(src: &RawVkPipelineLayout) -> VkPipelineLayout {
        VkPipelineLayout {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkPipelineLayout> for VkPipelineLayout {
    fn vk_to_raw(src: &VkPipelineLayout, dst: &mut RawVkPipelineLayout) {
        *dst = src._handle
    }
}

impl Default for VkPipelineLayout {
    fn default() -> VkPipelineLayout {
        VkPipelineLayout {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkPipelineLayout {
    fn eq(&self, other: &VkPipelineLayout) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkPipelineLayout {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkPipelineLayout {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Wrapper for [vkDestroyPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyPipelineLayout.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyPipelineLayout)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
}