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
use vulkan::vk::{VkAttachmentDescriptionFlags,RawVkAttachmentDescriptionFlags};
use vulkan::vk::{VkFormat,RawVkFormat};
use vulkan::vk::{VkSampleCountFlags,RawVkSampleCountFlags};
use vulkan::vk::{VkAttachmentLoadOp,RawVkAttachmentLoadOp};
use vulkan::vk::{VkAttachmentStoreOp,RawVkAttachmentStoreOp};
use vulkan::vk::{VkImageLayout,RawVkImageLayout};

/// Wrapper for [VkAttachmentDescription](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAttachmentDescription.html).
#[derive(Debug, Clone)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlags,
    pub load_op: VkAttachmentLoadOp,
    pub store_op: VkAttachmentStoreOp,
    pub stencil_load_op: VkAttachmentLoadOp,
    pub stencil_store_op: VkAttachmentStoreOp,
    pub initial_layout: VkImageLayout,
    pub final_layout: VkImageLayout,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkAttachmentDescription {
    pub flags: RawVkAttachmentDescriptionFlags,
    pub format: RawVkFormat,
    pub samples: RawVkSampleCountFlags,
    pub load_op: RawVkAttachmentLoadOp,
    pub store_op: RawVkAttachmentStoreOp,
    pub stencil_load_op: RawVkAttachmentLoadOp,
    pub stencil_store_op: RawVkAttachmentStoreOp,
    pub initial_layout: RawVkImageLayout,
    pub final_layout: RawVkImageLayout,
}

impl VkWrappedType<RawVkAttachmentDescription> for VkAttachmentDescription {
    fn vk_to_raw(src: &VkAttachmentDescription, dst: &mut RawVkAttachmentDescription) {
        dst.flags = vk_to_raw_value(&src.flags);
        dst.format = vk_to_raw_value(&src.format);
        dst.samples = vk_to_raw_value(&src.samples);
        dst.load_op = vk_to_raw_value(&src.load_op);
        dst.store_op = vk_to_raw_value(&src.store_op);
        dst.stencil_load_op = vk_to_raw_value(&src.stencil_load_op);
        dst.stencil_store_op = vk_to_raw_value(&src.stencil_store_op);
        dst.initial_layout = vk_to_raw_value(&src.initial_layout);
        dst.final_layout = vk_to_raw_value(&src.final_layout);
    }
}

impl VkRawType<VkAttachmentDescription> for RawVkAttachmentDescription {
    fn vk_to_wrapped(src: &RawVkAttachmentDescription) -> VkAttachmentDescription {
        VkAttachmentDescription {
            flags: RawVkAttachmentDescriptionFlags::vk_to_wrapped(&src.flags),
            format: RawVkFormat::vk_to_wrapped(&src.format),
            samples: RawVkSampleCountFlags::vk_to_wrapped(&src.samples),
            load_op: RawVkAttachmentLoadOp::vk_to_wrapped(&src.load_op),
            store_op: RawVkAttachmentStoreOp::vk_to_wrapped(&src.store_op),
            stencil_load_op: RawVkAttachmentLoadOp::vk_to_wrapped(&src.stencil_load_op),
            stencil_store_op: RawVkAttachmentStoreOp::vk_to_wrapped(&src.stencil_store_op),
            initial_layout: RawVkImageLayout::vk_to_wrapped(&src.initial_layout),
            final_layout: RawVkImageLayout::vk_to_wrapped(&src.final_layout),
        }
    }
}

impl Default for VkAttachmentDescription {
    fn default() -> VkAttachmentDescription {
        VkAttachmentDescription {
            flags: VkAttachmentDescriptionFlags::default(),
            format: VkFormat::default(),
            samples: VkSampleCountFlags::default(),
            load_op: VkAttachmentLoadOp::default(),
            store_op: VkAttachmentStoreOp::default(),
            stencil_load_op: VkAttachmentLoadOp::default(),
            stencil_store_op: VkAttachmentStoreOp::default(),
            initial_layout: VkImageLayout::default(),
            final_layout: VkImageLayout::default(),
        }
    }
}

impl VkSetup for VkAttachmentDescription {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkAttachmentDescription {
    fn vk_free(&mut self) {
        
    }
}