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
use vulkan::nv::{VkGeometryTriangles,RawVkGeometryTriangles};
use vulkan::nv::{VkGeometryAABB,RawVkGeometryAABB};

/// Wrapper for [VkGeometryDataNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkGeometryDataNV.html).
#[derive(Debug, Clone)]
pub struct VkGeometryData {
    pub triangles: VkGeometryTriangles,
    pub aabbs: VkGeometryAABB,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkGeometryData {
    pub triangles: RawVkGeometryTriangles,
    pub aabbs: RawVkGeometryAABB,
}

impl VkWrappedType<RawVkGeometryData> for VkGeometryData {
    fn vk_to_raw(src: &VkGeometryData, dst: &mut RawVkGeometryData) {
        dst.triangles = vk_to_raw_value(&src.triangles);
        dst.aabbs = vk_to_raw_value(&src.aabbs);
    }
}

impl VkRawType<VkGeometryData> for RawVkGeometryData {
    fn vk_to_wrapped(src: &RawVkGeometryData) -> VkGeometryData {
        VkGeometryData {
            triangles: RawVkGeometryTriangles::vk_to_wrapped(&src.triangles),
            aabbs: RawVkGeometryAABB::vk_to_wrapped(&src.aabbs),
        }
    }
}

impl Default for VkGeometryData {
    fn default() -> VkGeometryData {
        VkGeometryData {
            triangles: Default::default(),
            aabbs: Default::default(),
        }
    }
}

impl VkSetup for VkGeometryData {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.triangles, fn_table);
        VkSetup::vk_setup(&mut self.aabbs, fn_table);
    }
}

impl VkFree for RawVkGeometryData {
    fn vk_free(&self) {
        
    }
}