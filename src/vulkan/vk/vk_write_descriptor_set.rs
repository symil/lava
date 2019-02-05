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
use vulkan::vk::{VkDescriptorSet,RawVkDescriptorSet};
use vulkan::vk::{VkDescriptorType,RawVkDescriptorType};
use vulkan::vk::{VkDescriptorImageInfo,RawVkDescriptorImageInfo};
use vulkan::vk::{VkDescriptorBufferInfo,RawVkDescriptorBufferInfo};
use vulkan::vk::{VkBufferView,RawVkBufferView};

/// Wrapper for [VkWriteDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkWriteDescriptorSet.html).
#[derive(Debug, Clone)]
pub struct VkWriteDescriptorSet<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    where
        'c: 'b,
        'd: 'b,
        'f: 'e,
        'h: 'g,
{
    pub dst_set: &'a VkDescriptorSet,
    pub dst_binding: usize,
    pub dst_array_element: usize,
    pub descriptor_type: VkDescriptorType,
    pub image_info: &'b [VkDescriptorImageInfo<'c, 'd>],
    pub buffer_info: &'e [VkDescriptorBufferInfo<'f>],
    pub texel_buffer_view: &'g [&'h VkBufferView],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkWriteDescriptorSet {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub dst_set: RawVkDescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: RawVkDescriptorType,
    pub image_info: *mut RawVkDescriptorImageInfo,
    pub buffer_info: *mut RawVkDescriptorBufferInfo,
    pub texel_buffer_view: *mut RawVkBufferView,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> VkWrappedType<RawVkWriteDescriptorSet> for VkWriteDescriptorSet<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    where
        'c: 'b,
        'd: 'b,
        'f: 'e,
        'h: 'g,
{
    fn vk_to_raw(src: &VkWriteDescriptorSet, dst: &mut RawVkWriteDescriptorSet) {
        dst.s_type = vk_to_raw_value(&VkStructureType::WriteDescriptorSet);
        dst.next = ptr::null();
        dst.dst_set = vk_to_raw_value(src.dst_set);
        dst.dst_binding = vk_to_raw_value(&src.dst_binding);
        dst.dst_array_element = vk_to_raw_value(&src.dst_array_element);
        dst.descriptor_count = cmp::max(cmp::max(src.image_info.len(), src.buffer_info.len()), src.texel_buffer_view.len()) as u32;
        dst.descriptor_type = vk_to_raw_value(&src.descriptor_type);
        dst.image_info = new_ptr_vk_array(src.image_info);
        dst.buffer_info = new_ptr_vk_array(src.buffer_info);
        dst.texel_buffer_view = new_ptr_vk_array_from_ref(src.texel_buffer_view);
    }
}

impl Default for VkWriteDescriptorSet<'static, 'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkWriteDescriptorSet<'static, 'static, 'static, 'static, 'static, 'static, 'static, 'static> {
        VkWriteDescriptorSet {
            dst_set: vk_null_ref(),
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_type: VkDescriptorType::default(),
            image_info: &[],
            buffer_info: &[],
            texel_buffer_view: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> VkSetup for VkWriteDescriptorSet<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    where
        'c: 'b,
        'd: 'b,
        'f: 'e,
        'h: 'g,
{
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkWriteDescriptorSet {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.descriptor_count as usize, self.image_info);
        free_vk_ptr_array(self.descriptor_count as usize, self.buffer_info);
        free_ptr(self.texel_buffer_view);
    }
}