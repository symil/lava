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
pub type RawVkAccelerationStructure = u64;

/// Wrapper for [VkAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAccelerationStructureNV.html).
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructure {
    _handle: RawVkAccelerationStructure,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkAccelerationStructure> for RawVkAccelerationStructure {
    fn vk_to_wrapped(src: &RawVkAccelerationStructure) -> VkAccelerationStructure {
        VkAccelerationStructure {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkAccelerationStructure> for VkAccelerationStructure {
    fn vk_to_raw(src: &VkAccelerationStructure, dst: &mut RawVkAccelerationStructure) {
        *dst = src._handle
    }
}

impl Default for VkAccelerationStructure {
    fn default() -> VkAccelerationStructure {
        VkAccelerationStructure {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkAccelerationStructure {
    fn eq(&self, other: &VkAccelerationStructure) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkAccelerationStructure {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkAccelerationStructure {
    
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
    
    /// Wrapper for [vkDestroyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyAccelerationStructureNV.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyAccelerationStructureNV)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
    
    /// Wrapper for [vkGetAccelerationStructureHandleNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetAccelerationStructureHandleNV.html).
    pub fn get_handle(&self, data: &[c_void]) -> Result<(), VkResult> {
        unsafe {
            let raw_data_size = data.len();
            let raw_data = get_vec_ptr(data);
            let vk_result = ((&*self._fn_table).vkGetAccelerationStructureHandleNV)((*self._fn_table).device, self._handle, raw_data_size, raw_data);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
}