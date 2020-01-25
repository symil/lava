// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSamplerReductionMode](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSamplerReductionMode.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerReductionMode {
    WeightedAverage = 0,
    Min = 1,
    Max = 2,
}

#[doc(hidden)]
pub type RawVkSamplerReductionMode = i32;

impl VkWrappedType<RawVkSamplerReductionMode> for VkSamplerReductionMode {
    fn vk_to_raw(src: &VkSamplerReductionMode, dst: &mut RawVkSamplerReductionMode) {
        *dst = *src as i32
    }
}

impl VkRawType<VkSamplerReductionMode> for RawVkSamplerReductionMode {
    fn vk_to_wrapped(src: &RawVkSamplerReductionMode) -> VkSamplerReductionMode {
        unsafe {
            *((src as *const i32) as *const VkSamplerReductionMode)
        }
    }
}

impl Default for VkSamplerReductionMode {
    fn default() -> VkSamplerReductionMode {
        VkSamplerReductionMode::WeightedAverage
    }
}