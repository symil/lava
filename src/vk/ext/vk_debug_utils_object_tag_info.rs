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
use vk::vk_object_type::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDebugUtilsObjectTagInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub object_type: RawVkObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub tag: *const c_void,
}

#[derive(Debug, Clone)]
pub struct VkDebugUtilsObjectTagInfo<'a> {
    pub object_type: VkObjectType,
    pub object_handle: usize,
    pub tag_name: usize,
    pub tag_size: usize,
    pub tag: &'a c_void,
}

impl<'a> VkWrappedType<RawVkDebugUtilsObjectTagInfo> for VkDebugUtilsObjectTagInfo<'a> {
    fn vk_to_raw(src: &VkDebugUtilsObjectTagInfo, dst: &mut RawVkDebugUtilsObjectTagInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DebugUtilsObjectTagInfoExt);
        dst.next = ptr::null();
        dst.object_type = vk_to_raw_value(&src.object_type);
        dst.object_handle = vk_to_raw_value(&src.object_handle);
        dst.tag_name = vk_to_raw_value(&src.tag_name);
        dst.tag_size = src.tag_size;
        dst.tag = src.tag as *const c_void;
    }
}

impl Default for VkDebugUtilsObjectTagInfo<'static> {
    fn default() -> VkDebugUtilsObjectTagInfo<'static> {
        VkDebugUtilsObjectTagInfo {
            object_type: VkObjectType::default(),
            object_handle: 0,
            tag_name: 0,
            tag_size: 0,
            tag: &0,
        }
    }
}

impl<'a> VkSetup for VkDebugUtilsObjectTagInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDebugUtilsObjectTagInfo {
    fn vk_free(&mut self) {
        
    }
}