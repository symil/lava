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
use vulkan::nv::{VkAccelerationStructureType,RawVkAccelerationStructureType};
use vulkan::nv::{VkBuildAccelerationStructureFlags,RawVkBuildAccelerationStructureFlags};
use vulkan::nv::{VkGeometry,RawVkGeometry};

/// Wrapper for [VkAccelerationStructureInfoNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAccelerationStructureInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkAccelerationStructureInfo<'a, 'b, 'c, 'd, 'e>
    where
        'b: 'a,
        'c: 'a,
        'd: 'a,
        'e: 'a,
{
    pub type_: VkAccelerationStructureType,
    pub flags: VkBuildAccelerationStructureFlags,
    pub instance_count: usize,
    pub geometries: &'a [VkGeometry<'b, 'c, 'd, 'e>],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkAccelerationStructureInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub type_: RawVkAccelerationStructureType,
    pub flags: RawVkBuildAccelerationStructureFlags,
    pub instance_count: u32,
    pub geometry_count: u32,
    pub geometries: *mut RawVkGeometry,
}

impl<'a, 'b, 'c, 'd, 'e> VkWrappedType<RawVkAccelerationStructureInfo> for VkAccelerationStructureInfo<'a, 'b, 'c, 'd, 'e>
    where
        'b: 'a,
        'c: 'a,
        'd: 'a,
        'e: 'a,
{
    fn vk_to_raw(src: &VkAccelerationStructureInfo, dst: &mut RawVkAccelerationStructureInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::AccelerationStructureInfoNv);
        dst.next = ptr::null();
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.instance_count = vk_to_raw_value(&src.instance_count);
        dst.geometry_count = src.geometries.len() as u32;
        dst.geometries = new_ptr_vk_array(src.geometries);
    }
}

impl Default for VkAccelerationStructureInfo<'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkAccelerationStructureInfo<'static, 'static, 'static, 'static, 'static> {
        VkAccelerationStructureInfo {
            type_: VkAccelerationStructureType::default(),
            flags: VkBuildAccelerationStructureFlags::default(),
            instance_count: 0,
            geometries: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e> VkSetup for VkAccelerationStructureInfo<'a, 'b, 'c, 'd, 'e>
    where
        'b: 'a,
        'c: 'a,
        'd: 'a,
        'e: 'a,
{
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkAccelerationStructureInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.geometry_count as usize, self.geometries);
    }
}