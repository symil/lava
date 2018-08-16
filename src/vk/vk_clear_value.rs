// Copied from `scripts/static/`

use std::mem;
use utils::vk_traits::*;
use vk::vk_clear_color_value::*;
use vk::vk_clear_depth_stencil_value::*;

pub type RawVkClearValue = [u8; 12];

#[derive(Debug, Clone)]
pub enum VkClearValue {
    Color(VkClearColorValue),
    DepthStencil(VkClearDepthStencilValue)
}

impl VkWrappedType<RawVkClearValue> for VkClearValue {
    fn vk_to_raw(value: &VkClearValue, dst: &mut RawVkClearValue) {
        unsafe {
            *dst = match *value {
                VkClearValue::Color(ref color_value) => mem::transmute_copy(&color_value),
                VkClearValue::DepthStencil(ref depth_stencil_value) => mem::transmute_copy(&depth_stencil_value)
            }
        }
    }
}

impl Default for VkClearValue {
    fn default() -> VkClearValue {
        VkClearValue::Color(VkClearColorValue::U([0; 4]))
    }
}