use std::fmt::*;
use std::mem;
use utils::vk_traits::*;
use utils::vk_ptr::*;

#[doc(hidden)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union RawVkPerformanceValueData {
    pub value_32: u32,
    pub value_64: u64,
    pub value_float: f32,
    pub value_bool: u32,
    pub value_string: *const char
}

/// Wrapper for [VkPerformanceValueDataINTEL](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPerformanceValueDataINTEL.html)
#[derive(Debug, Clone)]
pub enum VkPerformanceValueData {
    U32(u32),
    U64(u64),
    F32(f32),
    B(bool),
    S(String)
}

impl Debug for RawVkPerformanceValueData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", unsafe { self.value_bool } )
    }
}

impl VkWrappedType<RawVkPerformanceValueData> for VkPerformanceValueData {
    fn vk_to_raw(value: &VkPerformanceValueData, dst: &mut RawVkPerformanceValueData) {
        match *value {
            VkPerformanceValueData::B(v) => {
                *dst = RawVkPerformanceValueData { value_bool: if v { 1 } else { 0 } };
            },
            VkPerformanceValueData::U32(v) => {
                *dst = RawVkPerformanceValueData { value_32: v };
            },
            VkPerformanceValueData::U64(v) => {
                *dst = RawVkPerformanceValueData { value_64: v };
            },
            VkPerformanceValueData::F32(v) => {
                *dst = RawVkPerformanceValueData { value_float: v };
            },
            VkPerformanceValueData::S(ref v) => {
                *dst = RawVkPerformanceValueData { value_string: new_ptr_string(v) as *const char };
            }
        }
    }
}

impl VkRawType<VkPerformanceValueData> for RawVkPerformanceValueData {
    fn vk_to_wrapped(value: &RawVkPerformanceValueData) -> VkPerformanceValueData {
        VkPerformanceValueData::U64(unsafe { value.value_64 })
    }
}

impl Default for VkPerformanceValueData {
    fn default() -> VkPerformanceValueData {
        VkPerformanceValueData::B(false)
    }
}