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
use vk::nv::vk_viewport_wscaling::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineViewportWScalingStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub viewport_wscaling_enable: u32,
    pub viewport_count: u32,
    pub viewport_wscalings: *mut RawVkViewportWScaling,
}

#[derive(Debug, Clone)]
pub struct VkPipelineViewportWScalingStateCreateInfo<'a> {
    pub viewport_wscaling_enable: bool,
    pub viewport_wscalings: &'a [VkViewportWScaling],
}

impl<'a> VkWrappedType<RawVkPipelineViewportWScalingStateCreateInfo> for VkPipelineViewportWScalingStateCreateInfo<'a> {
    fn vk_to_raw(src: &VkPipelineViewportWScalingStateCreateInfo, dst: &mut RawVkPipelineViewportWScalingStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineViewportWScalingStateCreateInfoNv);
        dst.next = ptr::null();
        dst.viewport_wscaling_enable = vk_to_raw_value(&src.viewport_wscaling_enable);
        dst.viewport_count = src.viewport_wscalings.len() as u32;
        dst.viewport_wscalings = new_ptr_vk_array(src.viewport_wscalings);
    }
}

impl Default for VkPipelineViewportWScalingStateCreateInfo<'static> {
    fn default() -> VkPipelineViewportWScalingStateCreateInfo<'static> {
        VkPipelineViewportWScalingStateCreateInfo {
            viewport_wscaling_enable: false,
            viewport_wscalings: &[],
        }
    }
}

impl<'a> VkSetup for VkPipelineViewportWScalingStateCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineViewportWScalingStateCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.viewport_count as usize, self.viewport_wscalings);
    }
}