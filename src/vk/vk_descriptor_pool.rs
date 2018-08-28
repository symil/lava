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

pub type RawVkDescriptorPool = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPool {
    _handle: RawVkDescriptorPool,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkDescriptorPool> for RawVkDescriptorPool {
    fn vk_to_wrapped(src: &RawVkDescriptorPool) -> VkDescriptorPool {
        VkDescriptorPool {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDescriptorPool> for VkDescriptorPool {
    fn vk_to_raw(src: &VkDescriptorPool, dst: &mut RawVkDescriptorPool) {
        *dst = src._handle
    }
}

impl Default for VkDescriptorPool {
    fn default() -> VkDescriptorPool {
        VkDescriptorPool {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDescriptorPool {
    fn eq(&self, other: &VkDescriptorPool) -> bool {
        self._handle == other._handle
    }
}

impl AsRef<VkDescriptorPool> for VkDescriptorPool {
    fn as_ref(&self) -> &VkDescriptorPool {
        self
    }
}

impl VkSetup for VkDescriptorPool {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkDescriptorPool {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyDescriptorPool)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn reset(&self, flags: VkDescriptorPoolResetFlags) -> Result<(), VkResult> {
        unsafe {
            let raw_flags = vk_to_raw_value(&flags);
            let vk_result = ((&*self._fn_table).vkResetDescriptorPool)(self._parent_device, self._handle, raw_flags);
            if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }
            Ok(())
        }
    }
    
    pub fn free_descriptor_sets(&self, descriptor_sets: &[&VkDescriptorSet]) -> Result<(), VkResult> {
        unsafe {
            let raw_descriptor_set_count = descriptor_sets.len() as u32;
            let raw_descriptor_sets = new_ptr_vk_array_from_ref(descriptor_sets);
            let vk_result = ((&*self._fn_table).vkFreeDescriptorSets)(self._parent_device, self._handle, raw_descriptor_set_count, raw_descriptor_sets);
            if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }
            free_ptr(raw_descriptor_sets);
            Ok(())
        }
    }
}