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
use vk::vk_pipeline_stage_flags::*;
use vk::vk_access_flags::*;
use vk::vk_dependency_flags::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassDependency2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: RawVkPipelineStageFlags,
    pub dst_stage_mask: RawVkPipelineStageFlags,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
    pub dependency_flags: RawVkDependencyFlags,
    pub view_offset: i32,
}

#[derive(Debug, Clone)]
pub struct VkSubpassDependency2 {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: VkPipelineStageFlags,
    pub dst_stage_mask: VkPipelineStageFlags,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub dependency_flags: VkDependencyFlags,
    pub view_offset: isize,
}

impl VkRawType<VkSubpassDependency2> for RawVkSubpassDependency2 {
    fn vk_to_wrapped(src: &RawVkSubpassDependency2) -> VkSubpassDependency2 {
        VkSubpassDependency2 {
            src_subpass: src.src_subpass,
            dst_subpass: src.dst_subpass,
            src_stage_mask: RawVkPipelineStageFlags::vk_to_wrapped(&src.src_stage_mask),
            dst_stage_mask: RawVkPipelineStageFlags::vk_to_wrapped(&src.dst_stage_mask),
            src_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.src_access_mask),
            dst_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.dst_access_mask),
            dependency_flags: RawVkDependencyFlags::vk_to_wrapped(&src.dependency_flags),
            view_offset: i32::vk_to_wrapped(&src.view_offset),
        }
    }
}

impl VkWrappedType<RawVkSubpassDependency2> for VkSubpassDependency2 {
    fn vk_to_raw(src: &VkSubpassDependency2, dst: &mut RawVkSubpassDependency2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassDependency2Khr);
        dst.next = ptr::null();
        dst.src_subpass = src.src_subpass;
        dst.dst_subpass = src.dst_subpass;
        dst.src_stage_mask = vk_to_raw_value(&src.src_stage_mask);
        dst.dst_stage_mask = vk_to_raw_value(&src.dst_stage_mask);
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
        dst.dependency_flags = vk_to_raw_value(&src.dependency_flags);
        dst.view_offset = vk_to_raw_value(&src.view_offset);
    }
}

impl Default for VkSubpassDependency2 {
    fn default() -> VkSubpassDependency2 {
        VkSubpassDependency2 {
            src_subpass: 0,
            dst_subpass: 0,
            src_stage_mask: VkPipelineStageFlags::default(),
            dst_stage_mask: VkPipelineStageFlags::default(),
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
            dependency_flags: VkDependencyFlags::default(),
            view_offset: 0,
        }
    }
}

impl VkSetup for VkSubpassDependency2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassDependency2 {
    fn vk_free(&mut self) {
        
    }
}