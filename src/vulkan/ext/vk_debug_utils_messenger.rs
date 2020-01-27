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
pub type RawVkDebugUtilsMessenger = u64;

/// Wrapper for [VkDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerEXT.html).
#[derive(Debug, Clone, Copy)]
pub struct VkDebugUtilsMessenger {
    _handle: RawVkDebugUtilsMessenger,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkDebugUtilsMessenger> for RawVkDebugUtilsMessenger {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessenger) -> VkDebugUtilsMessenger {
        VkDebugUtilsMessenger {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDebugUtilsMessenger> for VkDebugUtilsMessenger {
    fn vk_to_raw(src: &VkDebugUtilsMessenger, dst: &mut RawVkDebugUtilsMessenger) {
        *dst = src._handle
    }
}

impl Default for VkDebugUtilsMessenger {
    fn default() -> VkDebugUtilsMessenger {
        VkDebugUtilsMessenger {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDebugUtilsMessenger {
    fn eq(&self, other: &VkDebugUtilsMessenger) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkDebugUtilsMessenger {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkDebugUtilsMessenger {
    
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
    
    /// Wrapper for [vkDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyDebugUtilsMessengerEXT)((*self._fn_table).instance, self._handle, ptr::null());
        }
    }
}