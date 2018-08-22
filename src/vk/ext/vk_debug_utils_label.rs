// Generated by `scripts/generate_vk.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDebugUtilsLabel {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub label_name: *mut c_char,
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct VkDebugUtilsLabel<'a> {
    pub label_name: &'a str,
    pub color: [f32; 4],
}

impl<'a> VkWrappedType<RawVkDebugUtilsLabel> for VkDebugUtilsLabel<'a> {
    fn vk_to_raw(src: &VkDebugUtilsLabel, dst: &mut RawVkDebugUtilsLabel) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DebugUtilsLabelExt);
        dst.next = ptr::null();
        dst.label_name = new_ptr_string(src.label_name);
        dst.color = unsafe { let mut dst_array : [f32; 4] = mem::uninitialized(); to_array(&src.color, &mut dst_array); dst_array };
    }
}

impl Default for VkDebugUtilsLabel<'static> {
    fn default() -> VkDebugUtilsLabel<'static> {
        VkDebugUtilsLabel {
            label_name: "",
            color: [0.0; 4],
        }
    }
}

impl<'a> VkSetup for VkDebugUtilsLabel<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDebugUtilsLabel {
    fn vk_free(&mut self) {
        free_ptr(self.label_name);
    }
}