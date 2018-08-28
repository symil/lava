// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use vk::*;

pub type RawVkSemaphore = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkSemaphore {
    _handle: RawVkSemaphore,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkSemaphore> for RawVkSemaphore {
    fn vk_to_wrapped(src: &RawVkSemaphore) -> VkSemaphore {
        VkSemaphore {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
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
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkSemaphore {
    fn eq(&self, other: &VkSemaphore) -> bool {
        self._handle == other._handle
    }
}

impl AsRef<VkSemaphore> for VkSemaphore {
    fn as_ref(&self) -> &VkSemaphore {
        self
    }
}

impl VkSetup for VkSemaphore {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkSemaphore {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroySemaphore)(self._parent_device, self._handle, ptr::null());
        }
    }
}