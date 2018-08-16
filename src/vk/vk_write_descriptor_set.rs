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
use vk::vk_descriptor_set::*;
use vk::vk_descriptor_type::*;
use vk::vk_descriptor_image_info::*;
use vk::vk_descriptor_buffer_info::*;
use vk::vk_buffer_view::*;

#[repr(C)]
pub struct RawVkWriteDescriptorSet {
    s_type: RawVkStructureType,
    next: *const c_void,
    dst_set: RawVkDescriptorSet,
    dst_binding: u32,
    dst_array_element: u32,
    descriptor_count: u32,
    descriptor_type: RawVkDescriptorType,
    image_info: *mut RawVkDescriptorImageInfo,
    buffer_info: *mut RawVkDescriptorBufferInfo,
    texel_buffer_view: *mut RawVkBufferView,
}

#[derive(Debug, Clone)]
pub struct VkWriteDescriptorSet<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'c: 'b,
        'd: 'b,
        'f: 'e,
{
    pub dst_set: &'a VkDescriptorSet,
    pub dst_binding: usize,
    pub dst_array_element: usize,
    pub descriptor_type: VkDescriptorType,
    pub image_info: &'b [VkDescriptorImageInfo<'c, 'd>],
    pub buffer_info: &'e [VkDescriptorBufferInfo<'f>],
    pub texel_buffer_view: &'g [VkBufferView],
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkWrappedType<RawVkWriteDescriptorSet> for VkWriteDescriptorSet<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'c: 'b,
        'd: 'b,
        'f: 'e,
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
        dst.texel_buffer_view = new_ptr_vk_array(src.texel_buffer_view);
    }
}

impl Default for VkWriteDescriptorSet<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkWriteDescriptorSet<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
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

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkSetup for VkWriteDescriptorSet<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'c: 'b,
        'd: 'b,
        'f: 'e,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkWriteDescriptorSet {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.descriptor_count as usize, self.image_info);
        free_vk_ptr_array(self.descriptor_count as usize, self.buffer_info);
        free_ptr(self.texel_buffer_view);
    }
}