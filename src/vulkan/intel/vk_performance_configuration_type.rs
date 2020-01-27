// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPerformanceConfigurationTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPerformanceConfigurationType {
    CommandQueueMetricsDiscoveryActivated = 0,
}

#[doc(hidden)]
pub type RawVkPerformanceConfigurationType = i32;

impl VkWrappedType<RawVkPerformanceConfigurationType> for VkPerformanceConfigurationType {
    fn vk_to_raw(src: &VkPerformanceConfigurationType, dst: &mut RawVkPerformanceConfigurationType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPerformanceConfigurationType> for RawVkPerformanceConfigurationType {
    fn vk_to_wrapped(src: &RawVkPerformanceConfigurationType) -> VkPerformanceConfigurationType {
        unsafe {
            *((src as *const i32) as *const VkPerformanceConfigurationType)
        }
    }
}

impl Default for VkPerformanceConfigurationType {
    fn default() -> VkPerformanceConfigurationType {
        VkPerformanceConfigurationType::CommandQueueMetricsDiscoveryActivated
    }
}