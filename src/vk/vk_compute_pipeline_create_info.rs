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
use vk::vk_pipeline_create_flags::*;
use vk::vk_pipeline_shader_stage_create_info::*;
use vk::vk_pipeline_layout::*;
use vk::vk_pipeline::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkComputePipelineCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineCreateFlags,
    pub stage: RawVkPipelineShaderStageCreateInfo,
    pub layout: RawVkPipelineLayout,
    pub base_pipeline_handle: RawVkPipeline,
    pub base_pipeline_index: i32,
}

#[derive(Debug, Clone)]
pub struct VkComputePipelineCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'd: 'c,
        'e: 'c,
{
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo<'a, 'b, 'c, 'd, 'e>,
    pub layout: &'f VkPipelineLayout,
    pub base_pipeline_handle: Option<&'g VkPipeline>,
    pub base_pipeline_index: isize,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkWrappedType<RawVkComputePipelineCreateInfo> for VkComputePipelineCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'd: 'c,
        'e: 'c,
{
    fn vk_to_raw(src: &VkComputePipelineCreateInfo, dst: &mut RawVkComputePipelineCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ComputePipelineCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.stage = vk_to_raw_value(&src.stage);
        dst.layout = vk_to_raw_value(src.layout);
        dst.base_pipeline_handle = if src.base_pipeline_handle.is_some() { vk_to_raw_value(src.base_pipeline_handle.unwrap()) } else { 0 };
        dst.base_pipeline_index = vk_to_raw_value(&src.base_pipeline_index);
    }
}

impl Default for VkComputePipelineCreateInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkComputePipelineCreateInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
        VkComputePipelineCreateInfo {
            flags: VkPipelineCreateFlags::default(),
            stage: VkPipelineShaderStageCreateInfo::default(),
            layout: vk_null_ref(),
            base_pipeline_handle: None,
            base_pipeline_index: 0,
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkSetup for VkComputePipelineCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'd: 'c,
        'e: 'c,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkComputePipelineCreateInfo {
    fn vk_free(&mut self) {
        RawVkPipelineShaderStageCreateInfo::vk_free(&mut self.stage);
    }
}