// Generated by `scripts/generate.js`

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
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::ext::{VkDebugReportObjectType,RawVkDebugReportObjectType};

/// Wrapper for [VkDebugMarkerObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkDebugMarkerObjectTagInfo<'a> {
    pub object_type: VkDebugReportObjectType,
    pub object: usize,
    pub tag_name: usize,
    pub tag: &'a [c_void],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDebugMarkerObjectTagInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub object_type: RawVkDebugReportObjectType,
    pub object: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub tag: *mut c_void,
}

impl<'a> VkWrappedType<RawVkDebugMarkerObjectTagInfo> for VkDebugMarkerObjectTagInfo<'a> {
    fn vk_to_raw(src: &VkDebugMarkerObjectTagInfo, dst: &mut RawVkDebugMarkerObjectTagInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DebugMarkerObjectTagInfoExt);
        dst.next = ptr::null_mut();
        dst.object_type = vk_to_raw_value(&src.object_type);
        dst.object = vk_to_raw_value(&src.object);
        dst.tag_name = vk_to_raw_value(&src.tag_name);
        dst.tag_size = src.tag.len();
        dst.tag = get_vec_ptr(src.tag);
    }
}

impl<'a> VkRawType<VkDebugMarkerObjectTagInfo<'a>> for RawVkDebugMarkerObjectTagInfo {
    fn vk_to_wrapped(src: &RawVkDebugMarkerObjectTagInfo) -> VkDebugMarkerObjectTagInfo<'a> {
        VkDebugMarkerObjectTagInfo {
            object_type: RawVkDebugReportObjectType::vk_to_wrapped(&src.object_type),
            object: u64::vk_to_wrapped(&src.object),
            tag_name: u64::vk_to_wrapped(&src.tag_name),
            tag: slice_from_ptr(src.tag_size as usize, src.tag),
        }
    }
}

impl Default for VkDebugMarkerObjectTagInfo<'static> {
    fn default() -> VkDebugMarkerObjectTagInfo<'static> {
        VkDebugMarkerObjectTagInfo {
            object_type: Default::default(),
            object: 0,
            tag_name: 0,
            tag: &[],
        }
    }
}

impl<'a> VkSetup for VkDebugMarkerObjectTagInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDebugMarkerObjectTagInfo {
    fn vk_free(&self) {
        
    }
}