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
use vulkan::vk::{VkSubpassDescriptionFlags,RawVkSubpassDescriptionFlags};
use vulkan::vk::{VkPipelineBindPoint,RawVkPipelineBindPoint};
use vulkan::vk::{VkAttachmentReference2,RawVkAttachmentReference2};

/// Wrapper for [VkSubpassDescription2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription2.html).
#[derive(Debug, Clone)]
pub struct VkSubpassDescription2 {
    pub flags: VkSubpassDescriptionFlags,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub view_mask: u32,
    pub input_attachments: Vec<VkAttachmentReference2>,
    pub color_attachments: Vec<VkAttachmentReference2>,
    pub resolve_attachments: Option<Vec<VkAttachmentReference2>>,
    pub depth_stencil_attachment: Option<VkAttachmentReference2>,
    pub preserve_attachments: Vec<usize>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassDescription2 {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
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

impl VkWrappedType<RawVkSubpassDescription2> for VkSubpassDescription2 {
    fn vk_to_raw(src: &VkSubpassDescription2, dst: &mut RawVkSubpassDescription2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassDescription2);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.pipeline_bind_point = vk_to_raw_value(&src.pipeline_bind_point);
        dst.view_mask = src.view_mask;
        dst.input_attachment_count = src.input_attachments.len() as u32;
        dst.input_attachments = new_ptr_vk_array(&src.input_attachments);
        dst.color_attachment_count = cmp::max(src.color_attachments.len(), get_array_option_len(&src.resolve_attachments)) as u32;
        dst.color_attachments = new_ptr_vk_array(&src.color_attachments);
        dst.resolve_attachments = new_ptr_vk_array_checked(&src.resolve_attachments);
        dst.depth_stencil_attachment = new_ptr_vk_value_checked(&src.depth_stencil_attachment);
        dst.preserve_attachment_count = src.preserve_attachments.len() as u32;
        dst.preserve_attachments = new_ptr_vk_array(&src.preserve_attachments);
    }
}

impl VkRawType<VkSubpassDescription2> for RawVkSubpassDescription2 {
    fn vk_to_wrapped(src: &RawVkSubpassDescription2) -> VkSubpassDescription2 {
        VkSubpassDescription2 {
            flags: RawVkSubpassDescriptionFlags::vk_to_wrapped(&src.flags),
            pipeline_bind_point: RawVkPipelineBindPoint::vk_to_wrapped(&src.pipeline_bind_point),
            view_mask: src.view_mask,
            input_attachments: new_vk_array(src.input_attachment_count, src.input_attachments),
            color_attachments: new_vk_array(src.color_attachment_count, src.color_attachments),
            resolve_attachments: new_vk_array_checked(src.color_attachment_count, src.resolve_attachments),
            depth_stencil_attachment: new_vk_value_checked(src.depth_stencil_attachment),
            preserve_attachments: new_vk_array(src.preserve_attachment_count, src.preserve_attachments),
        }
    }
}

impl Default for VkSubpassDescription2 {
    fn default() -> VkSubpassDescription2 {
        VkSubpassDescription2 {
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            view_mask: 0,
            input_attachments: Vec::new(),
            color_attachments: Vec::new(),
            resolve_attachments: None,
            depth_stencil_attachment: None,
            preserve_attachments: Vec::new(),
        }
    }
}

impl VkSetup for VkSubpassDescription2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSubpassDescription2 {
    fn vk_free(&self) {
        free_vk_ptr_array(self.input_attachment_count as usize, self.input_attachments);
        free_vk_ptr_array(self.color_attachment_count as usize, self.color_attachments);
        free_vk_ptr_array(self.color_attachment_count as usize, self.resolve_attachments);
        free_vk_ptr(self.depth_stencil_attachment);
        free_ptr(self.preserve_attachments);
    }
}