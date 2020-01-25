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
pub type RawVkPerformanceConfiguration = u64;

/// Wrapper for [VkPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPerformanceConfigurationINTEL.html).
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceConfiguration {
    _handle: RawVkPerformanceConfiguration,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkPerformanceConfiguration> for RawVkPerformanceConfiguration {
    fn vk_to_wrapped(src: &RawVkPerformanceConfiguration) -> VkPerformanceConfiguration {
        VkPerformanceConfiguration {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkPerformanceConfiguration> for VkPerformanceConfiguration {
    fn vk_to_raw(src: &VkPerformanceConfiguration, dst: &mut RawVkPerformanceConfiguration) {
        *dst = src._handle
    }
}

impl Default for VkPerformanceConfiguration {
    fn default() -> VkPerformanceConfiguration {
        VkPerformanceConfiguration {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkPerformanceConfiguration {
    fn eq(&self, other: &VkPerformanceConfiguration) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkPerformanceConfiguration {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkPerformanceConfiguration {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Indicates if the Vulkan internal handle for this object is 0.
    pub fn is_null(&self) -> bool {
        self._handle == 0
    }
    
    /// Creates an object with a null Vulkan internal handle.
    ///
    /// Calling a method with a null handle will most likely result in a crash.
    pub fn null() -> Self {
        Self {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}