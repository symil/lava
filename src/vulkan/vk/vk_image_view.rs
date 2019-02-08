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
pub type RawVkImageView = u64;

/// Wrapper for [VkImageView](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageView.html).
#[derive(Debug, Clone, Copy)]
pub struct VkImageView {
    _handle: RawVkImageView,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkImageView> for RawVkImageView {
    fn vk_to_wrapped(src: &RawVkImageView) -> VkImageView {
        VkImageView {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkImageView> for VkImageView {
    fn vk_to_raw(src: &VkImageView, dst: &mut RawVkImageView) {
        *dst = src._handle
    }
}

impl Default for VkImageView {
    fn default() -> VkImageView {
        VkImageView {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkImageView {
    fn eq(&self, other: &VkImageView) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkImageView {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkImageView {
    
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
    
    /// Wrapper for [vkDestroyImageView](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyImageView.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyImageView)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
}