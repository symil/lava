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
pub type RawVkQueue = u64;

/// Wrapper for [VkQueue](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueue.html).
#[derive(Debug, Clone, Copy)]
pub struct VkQueue {
    _handle: RawVkQueue,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkQueue> for RawVkQueue {
    fn vk_to_wrapped(src: &RawVkQueue) -> VkQueue {
        VkQueue {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkQueue> for VkQueue {
    fn vk_to_raw(src: &VkQueue, dst: &mut RawVkQueue) {
        *dst = src._handle
    }
}

impl Default for VkQueue {
    fn default() -> VkQueue {
        VkQueue {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkQueue {
    fn eq(&self, other: &VkQueue) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkQueue {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkQueue {
    
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
    
    /// Wrapper for [vkQueueSubmit](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueSubmit.html).
    pub fn submit(&self, submits: Vec<VkSubmitInfo>, fence: Option<VkFence>) -> Result<(), VkResult> {
        unsafe {
            let raw_submit_count = submits.len() as u32;
            let raw_submits = new_ptr_vk_array(&submits);
            let raw_fence = vk_to_raw_value_checked(&fence);
            let vk_result = ((&*self._fn_table).vkQueueSubmit)(self._handle, raw_submit_count, raw_submits, raw_fence);
            free_vk_ptr_array(raw_submit_count as usize, raw_submits);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkQueueWaitIdle](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueWaitIdle.html).
    pub fn wait_idle(&self) -> Result<(), VkResult> {
        unsafe {
            let vk_result = ((&*self._fn_table).vkQueueWaitIdle)(self._handle);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkQueueBindSparse](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueBindSparse.html).
    pub fn bind_sparse(&self, bind_info: Vec<VkBindSparseInfo>, fence: Option<VkFence>) -> Result<(), VkResult> {
        unsafe {
            let raw_bind_info_count = bind_info.len() as u32;
            let raw_bind_info = new_ptr_vk_array(&bind_info);
            let raw_fence = vk_to_raw_value_checked(&fence);
            let vk_result = ((&*self._fn_table).vkQueueBindSparse)(self._handle, raw_bind_info_count, raw_bind_info, raw_fence);
            free_vk_ptr_array(raw_bind_info_count as usize, raw_bind_info);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkQueuePresentKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueuePresentKHR.html).
    pub fn present(&self, present_info: khr::VkPresentInfo) -> Result<(), VkResult> {
        unsafe {
            let raw_present_info = new_ptr_vk_value(&present_info);
            let vk_result = ((&*self._fn_table).vkQueuePresentKHR)(self._handle, raw_present_info);
            free_vk_ptr(raw_present_info);
            if vk_result == 0 { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }
        }
    }
    
    /// Wrapper for [vkQueueBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html).
    pub fn begin_debug_utils_label(&self, label_info: ext::VkDebugUtilsLabel) {
        unsafe {
            let raw_label_info = new_ptr_vk_value(&label_info);
            ((&*self._fn_table).vkQueueBeginDebugUtilsLabelEXT)(self._handle, raw_label_info);
            free_vk_ptr(raw_label_info);
        }
    }
    
    /// Wrapper for [vkQueueEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html).
    pub fn end_debug_utils_label(&self) {
        unsafe {
            ((&*self._fn_table).vkQueueEndDebugUtilsLabelEXT)(self._handle);
        }
    }
    
    /// Wrapper for [vkQueueInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html).
    pub fn insert_debug_utils_label(&self, label_info: ext::VkDebugUtilsLabel) {
        unsafe {
            let raw_label_info = new_ptr_vk_value(&label_info);
            ((&*self._fn_table).vkQueueInsertDebugUtilsLabelEXT)(self._handle, raw_label_info);
            free_vk_ptr(raw_label_info);
        }
    }
    
    /// Wrapper for [vkGetQueueCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetQueueCheckpointDataNV.html).
    pub fn get_checkpoint_data(&self) -> Vec<nv::VkCheckpointData> {
        unsafe {
            let mut raw_checkpoint_data : *mut nv::RawVkCheckpointData = ptr::null_mut();
            let raw_checkpoint_data_count = &mut mem::zeroed() as *mut u32;
            ((&*self._fn_table).vkGetQueueCheckpointDataNV)(self._handle, raw_checkpoint_data_count, raw_checkpoint_data);
            raw_checkpoint_data = calloc(*raw_checkpoint_data_count as usize, mem::size_of::<nv::RawVkCheckpointData>()) as *mut nv::RawVkCheckpointData;
            
            ((&*self._fn_table).vkGetQueueCheckpointDataNV)(self._handle, raw_checkpoint_data_count, raw_checkpoint_data);
            
            let mut checkpoint_data = new_vk_array(*raw_checkpoint_data_count, raw_checkpoint_data);
            for elt in &mut checkpoint_data { VkSetup::vk_setup(elt, self._fn_table); }
            free(raw_checkpoint_data as *mut u8);
            checkpoint_data
        }
    }
}