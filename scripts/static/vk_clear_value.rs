use std::mem;
use utils::vk_type::*;
use vk::vk_clear_color_value::*;
use vk::vk_clear_depth_stencil_value::*;

pub type RawVkClearValue = [u8; 12];

#[derive(Debug, Clone, Copy)]
pub enum VkClearValue {
    Color(VkClearColorValue),
    DepthStencil(VkClearDepthStencilValue)
}

impl VkWrappedType<RawVkClearValue> for VkClearValue {
    fn vk_to_raw(value: &VkClearValue, dst: &mut RawVkClearValue) {
        unsafe {
            *dst = match *value {
                VkClearValue::Color(color_value) => mem::transmute_copy(&color_value),
                VkClearValue::DepthStencil(depth_stencil_value) => mem::transmute_copy(&depth_stencil_value)
            }
        }
    }
}

impl VkDefault for VkClearValue {
    fn vk_default() -> VkClearValue {
        VkClearValue::Color(VkClearColorValue::U([0; 4]))
    }
}