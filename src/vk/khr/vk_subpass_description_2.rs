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
use vk::vk_subpass_description_flags::*;
use vk::vk_pipeline_bind_point::*;
use vk::khr::vk_attachment_reference_2::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassDescription2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkSubpassDescriptionFlags,
    pub pipeline_bind_point: RawVkPipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub input_attachments: *mut RawVkAttachmentReference2,
    pub color_attachment_count: u32,
    pub color_attachments: *mut RawVkAttachmentReference2,
    pub resolve_attachments: *mut RawVkAttachmentReference2,
    pub depth_stencil_attachment: *mut RawVkAttachmentReference2,
    pub preserve_attachment_count: u32,
    pub preserve_attachments: *mut u32,
}

#[derive(Debug, Clone)]
pub struct VkSubpassDescription2<'a, 'b, 'c, 'd, 'e> {
    pub flags: VkSubpassDescriptionFlags,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub view_mask: u32,
    pub input_attachments: &'a [VkAttachmentReference2],
    pub color_attachments: &'b [VkAttachmentReference2],
    pub resolve_attachments: Option<&'c [VkAttachmentReference2]>,
    pub depth_stencil_attachment: Option<&'d VkAttachmentReference2>,
    pub preserve_attachments: &'e [usize],
}

impl<'a, 'b, 'c, 'd, 'e> VkWrappedType<RawVkSubpassDescription2> for VkSubpassDescription2<'a, 'b, 'c, 'd, 'e> {
    fn vk_to_raw(src: &VkSubpassDescription2, dst: &mut RawVkSubpassDescription2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassDescription2Khr);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.pipeline_bind_point = vk_to_raw_value(&src.pipeline_bind_point);
        dst.view_mask = src.view_mask;
        dst.input_attachment_count = src.input_attachments.len() as u32;
        dst.input_attachments = new_ptr_vk_array(src.input_attachments);
        dst.color_attachment_count = cmp::max(src.color_attachments.len(), get_array_option_len(src.resolve_attachments)) as u32;
        dst.color_attachments = new_ptr_vk_array(src.color_attachments);
        dst.resolve_attachments = new_ptr_vk_array_checked(src.resolve_attachments);
        dst.depth_stencil_attachment = new_ptr_vk_value_checked(src.depth_stencil_attachment);
        dst.preserve_attachment_count = src.preserve_attachments.len() as u32;
        dst.preserve_attachments = new_ptr_vk_array(src.preserve_attachments);
    }
}

impl Default for VkSubpassDescription2<'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkSubpassDescription2<'static, 'static, 'static, 'static, 'static> {
        VkSubpassDescription2 {
            flags: VkSubpassDescriptionFlags::default(),
            pipeline_bind_point: VkPipelineBindPoint::default(),
            view_mask: 0,
            input_attachments: &[],
            color_attachments: &[],
            resolve_attachments: None,
            depth_stencil_attachment: None,
            preserve_attachments: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e> VkSetup for VkSubpassDescription2<'a, 'b, 'c, 'd, 'e> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassDescription2 {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.input_attachment_count as usize, self.input_attachments);
        free_vk_ptr_array(self.color_attachment_count as usize, self.color_attachments);
        free_vk_ptr_array(self.color_attachment_count as usize, self.resolve_attachments);
        free_vk_ptr(self.depth_stencil_attachment);
        free_ptr(self.preserve_attachments);
    }
}