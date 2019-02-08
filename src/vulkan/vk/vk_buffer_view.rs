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
pub type RawVkBufferView = u64;

/// Wrapper for [VkBufferView](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferView.html).
#[derive(Debug, Clone, Copy)]
pub struct VkBufferView {
    _handle: RawVkBufferView,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkBufferView> for RawVkBufferView {
    fn vk_to_wrapped(src: &RawVkBufferView) -> VkBufferView {
        VkBufferView {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkBufferView> for VkBufferView {
    fn vk_to_raw(src: &VkBufferView, dst: &mut RawVkBufferView) {
        *dst = src._handle
    }
}

impl Default for VkBufferView {
    fn default() -> VkBufferView {
        VkBufferView {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkBufferView {
    fn eq(&self, other: &VkBufferView) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkBufferView {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkBufferView {
    
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
    
    /// Wrapper for [vkDestroyBufferView](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyBufferView.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyBufferView)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
}