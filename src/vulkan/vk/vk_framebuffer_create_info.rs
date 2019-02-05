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
use vulkan::vk::{VkFramebufferCreateFlags,RawVkFramebufferCreateFlags};
use vulkan::vk::{VkRenderPass,RawVkRenderPass};
use vulkan::vk::{VkImageView,RawVkImageView};

/// Wrapper for [VkFramebufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFramebufferCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkFramebufferCreateInfo {
    pub flags: VkFramebufferCreateFlags,
    pub render_pass: VkRenderPass,
    pub attachments: Vec<VkImageView>,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkFramebufferCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkFramebufferCreateFlags,
    pub render_pass: RawVkRenderPass,
    pub attachment_count: u32,
    pub attachments: *mut RawVkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl VkWrappedType<RawVkFramebufferCreateInfo> for VkFramebufferCreateInfo {
    fn vk_to_raw(src: &VkFramebufferCreateInfo, dst: &mut RawVkFramebufferCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FramebufferCreateInfo);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.render_pass = vk_to_raw_value(&src.render_pass);
        dst.attachment_count = src.attachments.len() as u32;
        dst.attachments = new_ptr_vk_array(&src.attachments);
        dst.width = src.width;
        dst.height = src.height;
        dst.layers = src.layers;
    }
}

impl VkRawType<VkFramebufferCreateInfo> for RawVkFramebufferCreateInfo {
    fn vk_to_wrapped(src: &RawVkFramebufferCreateInfo) -> VkFramebufferCreateInfo {
        VkFramebufferCreateInfo {
            flags: RawVkFramebufferCreateFlags::vk_to_wrapped(&src.flags),
            render_pass: RawVkRenderPass::vk_to_wrapped(&src.render_pass),
            attachments: new_vk_array(src.attachment_count, src.attachments),
            width: src.width,
            height: src.height,
            layers: src.layers,
        }
    }
}

impl Default for VkFramebufferCreateInfo {
    fn default() -> VkFramebufferCreateInfo {
        VkFramebufferCreateInfo {
            flags: Default::default(),
            render_pass: Default::default(),
            attachments: Vec::new(),
            width: 0,
            height: 0,
            layers: 0,
        }
    }
}

impl VkSetup for VkFramebufferCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.render_pass, fn_table);
    }
}

impl VkFree for RawVkFramebufferCreateInfo {
    fn vk_free(&self) {
        free_ptr(self.attachments);
    }
}