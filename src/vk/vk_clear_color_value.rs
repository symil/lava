// Copied from `scripts/static/`

use std::fmt::*;
use std::mem;
use utils::vk_traits::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub union RawVkClearColorValue {
    float32: [f32; 4],
    int32: [i32; 4],
    uint32: [u32; 4]
}

#[derive(Debug, Clone)]
pub enum VkClearColorValue {
    F([f32; 4]),
    I([i32; 4]),
    U([u32; 4])
}

impl Debug for RawVkClearColorValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", unsafe { self.float32 } )
    }
}

impl VkWrappedType<RawVkClearColorValue> for VkClearColorValue {
    fn vk_to_raw(value: &VkClearColorValue, dst: &mut RawVkClearColorValue) {
        match *value {
            VkClearColorValue::F(array) => {
                *dst = RawVkClearColorValue { float32: array };
            },
            VkClearColorValue::I(array) => {
                *dst = RawVkClearColorValue { int32: array };
            },
            VkClearColorValue::U(array) => {
                *dst = RawVkClearColorValue { uint32: array };
            }
        }
    }
}

impl Default for VkClearColorValue {
    fn default() -> VkClearColorValue {
        VkClearColorValue::U([0; 4])
    }
}