// Copied from `scripts/static/`

use std::mem;
use utils::vk_traits::*;

pub type RawVkClearColorValue = [u32; 4];

#[derive(Debug, Clone)]
pub enum VkClearColorValue {
    F([f32; 4]),
    I([i32; 4]),
    U([u32; 4])
}

impl VkWrappedType<RawVkClearColorValue> for VkClearColorValue {
    fn vk_to_raw(value: &VkClearColorValue, dst: &mut RawVkClearColorValue) {
        unsafe {
            *dst = match *value {
                VkClearColorValue::F(array) => mem::transmute_copy(&array),
                VkClearColorValue::I(array) => mem::transmute_copy(&array),
                VkClearColorValue::U(array) => array
            }
        }
    }
}

impl Default for VkClearColorValue {
    fn default() -> VkClearColorValue {
        VkClearColorValue::U([0; 4])
    }
}