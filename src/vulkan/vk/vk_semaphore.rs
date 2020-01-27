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
pub type RawVkSemaphore = u64;

/// Wrapper for [VkSemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphore.html).
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphore {
    _handle: RawVkSemaphore,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkSemaphore> for RawVkSemaphore {
    fn vk_to_wrapped(src: &RawVkSemaphore) -> VkSemaphore {
        VkSemaphore {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkSemaphore> for VkSemaphore {
    fn vk_to_raw(src: &VkSemaphore, dst: &mut RawVkSemaphore) {
        *dst = src._handle
    }
}

impl Default for VkSemaphore {
    fn default() -> VkSemaphore {
        VkSemaphore {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkSemaphore {
    fn eq(&self, other: &VkSemaphore) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkSemaphore {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkSemaphore {
    
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
    
    /// Wrapper for [vkDestroySemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySemaphore.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroySemaphore)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetSemaphoreCounterValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html).
    pub fn get_counter_value(&self) -> LavaResult<usize> {
        unsafe {
            let mut vk_result = 0;
            let raw_value = &mut mem::zeroed() as *mut u64;
            
            vk_result = ((&*self._fn_table).vkGetSemaphoreCounterValue)((*self._fn_table).device, self._handle, raw_value);
            
            let value = new_vk_value(raw_value);
            if vk_result == 0 { Ok(value) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), value)) }
        }
    }
}