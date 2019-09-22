// Copied from `scripts/static/`

use std::fmt::*;
use std::mem;
use utils::vk_traits::*;

#[doc(hidden)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union RawVkPipelineExecutableStatisticValue {
    pub b32_: u32,
    pub i64_: i64,
    pub u64_: u64,
    pub f64_: f64
}

/// Wrapper for [VkPipelineExecutableStatisticValueKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html)
#[derive(Debug, Clone)]
pub enum VkPipelineExecutableStatisticValue {
    B(bool),
    I(i64),
    U(u64),
    F(f64)
}

impl Debug for RawVkPipelineExecutableStatisticValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", unsafe { self.f64_ } )
    }
}

impl VkWrappedType<RawVkPipelineExecutableStatisticValue> for VkPipelineExecutableStatisticValue {
    fn vk_to_raw(value: &VkPipelineExecutableStatisticValue, dst: &mut RawVkPipelineExecutableStatisticValue) {
        match *value {
            VkPipelineExecutableStatisticValue::B(v) => {
                *dst = RawVkPipelineExecutableStatisticValue { b32_: if v { 1 } else { 0 } };
            },
            VkPipelineExecutableStatisticValue::I(v) => {
                *dst = RawVkPipelineExecutableStatisticValue { i64_: v };
            },
            VkPipelineExecutableStatisticValue::U(v) => {
                *dst = RawVkPipelineExecutableStatisticValue { u64_: v };
            },
            VkPipelineExecutableStatisticValue::F(v) => {
                *dst = RawVkPipelineExecutableStatisticValue { f64_: v };
            }
        }
    }
}

impl VkRawType<VkPipelineExecutableStatisticValue> for RawVkPipelineExecutableStatisticValue {
    fn vk_to_wrapped(value: &RawVkPipelineExecutableStatisticValue) -> VkPipelineExecutableStatisticValue {
        VkPipelineExecutableStatisticValue::U(unsafe { value.u64_ })
    }
}

impl Default for VkPipelineExecutableStatisticValue {
    fn default() -> VkPipelineExecutableStatisticValue {
        VkPipelineExecutableStatisticValue::B(false)
    }
}