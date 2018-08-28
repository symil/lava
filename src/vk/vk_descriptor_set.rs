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

pub type RawVkDescriptorSet = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSet {
    _handle: RawVkDescriptorSet,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkDescriptorSet> for RawVkDescriptorSet {
    fn vk_to_wrapped(src: &RawVkDescriptorSet) -> VkDescriptorSet {
        VkDescriptorSet {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDescriptorSet> for VkDescriptorSet {
    fn vk_to_raw(src: &VkDescriptorSet, dst: &mut RawVkDescriptorSet) {
        *dst = src._handle
    }
}

impl Default for VkDescriptorSet {
    fn default() -> VkDescriptorSet {
        VkDescriptorSet {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDescriptorSet {
    fn eq(&self, other: &VkDescriptorSet) -> bool {
        self._handle == other._handle
    }
}

impl AsRef<VkDescriptorSet> for VkDescriptorSet {
    fn as_ref(&self) -> &VkDescriptorSet {
        self
    }
}

impl VkSetup for VkDescriptorSet {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkDescriptorSet {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
}