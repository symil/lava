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
use vk::vk_render_pass::*;
use vk::vk_framebuffer::*;
use vk::vk_query_control_flags::*;
use vk::vk_query_pipeline_statistic_flags::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandBufferInheritanceInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub render_pass: RawVkRenderPass,
    pub subpass: u32,
    pub framebuffer: RawVkFramebuffer,
    pub occlusion_query_enable: u32,
    pub query_flags: RawVkQueryControlFlags,
    pub pipeline_statistics: RawVkQueryPipelineStatisticFlags,
}

#[derive(Debug, Clone)]
pub struct VkCommandBufferInheritanceInfo<'a, 'b> {
    pub render_pass: Option<&'a VkRenderPass>,
    pub subpass: u32,
    pub framebuffer: Option<&'b VkFramebuffer>,
    pub occlusion_query_enable: bool,
    pub query_flags: VkQueryControlFlags,
    pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

impl<'a, 'b> VkWrappedType<RawVkCommandBufferInheritanceInfo> for VkCommandBufferInheritanceInfo<'a, 'b> {
    fn vk_to_raw(src: &VkCommandBufferInheritanceInfo, dst: &mut RawVkCommandBufferInheritanceInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandBufferInheritanceInfo);
        dst.next = ptr::null();
        dst.render_pass = if src.render_pass.is_some() { vk_to_raw_value(src.render_pass.unwrap()) } else { 0 };
        dst.subpass = src.subpass;
        dst.framebuffer = if src.framebuffer.is_some() { vk_to_raw_value(src.framebuffer.unwrap()) } else { 0 };
        dst.occlusion_query_enable = vk_to_raw_value(&src.occlusion_query_enable);
        dst.query_flags = vk_to_raw_value(&src.query_flags);
        dst.pipeline_statistics = vk_to_raw_value(&src.pipeline_statistics);
    }
}

impl Default for VkCommandBufferInheritanceInfo<'static, 'static> {
    fn default() -> VkCommandBufferInheritanceInfo<'static, 'static> {
        VkCommandBufferInheritanceInfo {
            render_pass: None,
            subpass: 0,
            framebuffer: None,
            occlusion_query_enable: false,
            query_flags: VkQueryControlFlags::default(),
            pipeline_statistics: VkQueryPipelineStatisticFlags::default(),
        }
    }
}

impl<'a, 'b> VkSetup for VkCommandBufferInheritanceInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCommandBufferInheritanceInfo {
    fn vk_free(&mut self) {
        
    }
}