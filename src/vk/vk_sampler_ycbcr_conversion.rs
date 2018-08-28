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

pub type RawVkSamplerYcbcrConversion = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkSamplerYcbcrConversion {
    _handle: RawVkSamplerYcbcrConversion,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkSamplerYcbcrConversion> for RawVkSamplerYcbcrConversion {
    fn vk_to_wrapped(src: &RawVkSamplerYcbcrConversion) -> VkSamplerYcbcrConversion {
        VkSamplerYcbcrConversion {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkSamplerYcbcrConversion> for VkSamplerYcbcrConversion {
    fn vk_to_raw(src: &VkSamplerYcbcrConversion, dst: &mut RawVkSamplerYcbcrConversion) {
        *dst = src._handle
    }
}

impl Default for VkSamplerYcbcrConversion {
    fn default() -> VkSamplerYcbcrConversion {
        VkSamplerYcbcrConversion {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkSamplerYcbcrConversion {
    fn eq(&self, other: &VkSamplerYcbcrConversion) -> bool {
        self._handle == other._handle
    }
}

impl AsRef<VkSamplerYcbcrConversion> for VkSamplerYcbcrConversion {
    fn as_ref(&self) -> &VkSamplerYcbcrConversion {
        self
    }
}

impl VkSetup for VkSamplerYcbcrConversion {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkSamplerYcbcrConversion {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroySamplerYcbcrConversion)(self._parent_device, self._handle, ptr::null());
        }
    }
}