use std::mem;
use utils::vk_type::*;

pub type RawVkClearColorValue = [u32; 4];

#[derive(Debug, Clone, Copy)]
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

impl VkDefault for VkClearColorValue {
    fn vk_default() -> VkClearColorValue {
        VkClearColorValue::U([0; 4])
    }
}