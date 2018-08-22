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
use vk::ext::vk_pipeline_discard_rectangle_state_create_flags::*;
use vk::ext::vk_discard_rectangle_mode::*;
use vk::vk_rect_2d::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineDiscardRectangleStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineDiscardRectangleStateCreateFlags,
    pub discard_rectangle_mode: RawVkDiscardRectangleMode,
    pub discard_rectangle_count: u32,
    pub discard_rectangles: *mut RawVkRect2D,
}

#[derive(Debug, Clone)]
pub struct VkPipelineDiscardRectangleStateCreateInfo<'a> {
    pub flags: VkPipelineDiscardRectangleStateCreateFlags,
    pub discard_rectangle_mode: VkDiscardRectangleMode,
    pub discard_rectangles: Option<&'a [VkRect2D]>,
}

impl<'a> VkWrappedType<RawVkPipelineDiscardRectangleStateCreateInfo> for VkPipelineDiscardRectangleStateCreateInfo<'a> {
    fn vk_to_raw(src: &VkPipelineDiscardRectangleStateCreateInfo, dst: &mut RawVkPipelineDiscardRectangleStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineDiscardRectangleStateCreateInfoExt);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.discard_rectangle_mode = vk_to_raw_value(&src.discard_rectangle_mode);
        dst.discard_rectangle_count = get_array_option_len(src.discard_rectangles) as u32;
        dst.discard_rectangles = new_ptr_vk_array_checked(src.discard_rectangles);
    }
}

impl Default for VkPipelineDiscardRectangleStateCreateInfo<'static> {
    fn default() -> VkPipelineDiscardRectangleStateCreateInfo<'static> {
        VkPipelineDiscardRectangleStateCreateInfo {
            flags: VkPipelineDiscardRectangleStateCreateFlags::default(),
            discard_rectangle_mode: VkDiscardRectangleMode::default(),
            discard_rectangles: None,
        }
    }
}

impl<'a> VkSetup for VkPipelineDiscardRectangleStateCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineDiscardRectangleStateCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.discard_rectangle_count as usize, self.discard_rectangles);
    }
}