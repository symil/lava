// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPerformanceValueTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueTypeINTEL.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPerformanceValueType {
    Uint32 = 0,
    Uint64 = 1,
    Float = 2,
    Bool = 3,
    String = 4,
}

#[doc(hidden)]
pub type RawVkPerformanceValueType = i32;

impl VkWrappedType<RawVkPerformanceValueType> for VkPerformanceValueType {
    fn vk_to_raw(src: &VkPerformanceValueType, dst: &mut RawVkPerformanceValueType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPerformanceValueType> for RawVkPerformanceValueType {
    fn vk_to_wrapped(src: &RawVkPerformanceValueType) -> VkPerformanceValueType {
        unsafe {
            *((src as *const i32) as *const VkPerformanceValueType)
        }
    }
}

impl Default for VkPerformanceValueType {
    fn default() -> VkPerformanceValueType {
        VkPerformanceValueType::Uint32
    }
}