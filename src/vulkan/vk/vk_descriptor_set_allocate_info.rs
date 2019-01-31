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
use vulkan::vk::{VkDescriptorPool,RawVkDescriptorPool};
use vulkan::vk::{VkDescriptorSetLayout,RawVkDescriptorSetLayout};

/// Wrapper for [VkDescriptorSetAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetAllocateInfo.html)
#[derive(Debug, Clone)]
pub struct VkDescriptorSetAllocateInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    pub descriptor_pool: &'a VkDescriptorPool,
    pub set_layouts: &'b [&'c VkDescriptorSetLayout],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorSetAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub descriptor_pool: RawVkDescriptorPool,
    pub descriptor_set_count: u32,
    pub set_layouts: *mut RawVkDescriptorSetLayout,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkDescriptorSetAllocateInfo> for VkDescriptorSetAllocateInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    fn vk_to_raw(src: &VkDescriptorSetAllocateInfo, dst: &mut RawVkDescriptorSetAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorSetAllocateInfo);
        dst.next = ptr::null();
        dst.descriptor_pool = vk_to_raw_value(src.descriptor_pool);
        dst.descriptor_set_count = src.set_layouts.len() as u32;
        dst.set_layouts = new_ptr_vk_array_from_ref(src.set_layouts);
    }
}

impl Default for VkDescriptorSetAllocateInfo<'static, 'static, 'static> {
    fn default() -> VkDescriptorSetAllocateInfo<'static, 'static, 'static> {
        VkDescriptorSetAllocateInfo {
            descriptor_pool: vk_null_ref(),
            set_layouts: &[],
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkDescriptorSetAllocateInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorSetAllocateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.set_layouts);
    }
}