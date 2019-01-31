use std::fmt::*;
use std::mem;
use utils::vk_traits::*;
use utils::vk_convert::*;
use vulkan::vk::*;

#[doc(hidden)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union RawVkClearValue {
    pub color: RawVkClearColorValue,
    pub depth_stencil: RawVkClearDepthStencilValue
}

/// Wrapper for [VkClearValue](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkClearValue.html)
#[derive(Debug, Clone)]
pub enum VkClearValue {
    Color(VkClearColorValue),
    DepthStencil(VkClearDepthStencilValue)
}

impl Debug for RawVkClearValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", unsafe { self.color })
    }
}

impl VkWrappedType<RawVkClearValue> for VkClearValue {
    fn vk_to_raw(value: &VkClearValue, dst: &mut RawVkClearValue) {
        match *value {
            VkClearValue::Color(ref color_value) => {
                *dst = RawVkClearValue { color: vk_to_raw_value(color_value) };
            },
            VkClearValue::DepthStencil(ref depth_stencil_value) => {
                *dst = RawVkClearValue { depth_stencil: vk_to_raw_value(depth_stencil_value) };
            }
        }
    }
}

impl Default for VkClearValue {
    fn default() -> VkClearValue {
        VkClearValue::Color(VkClearColorValue::U([0; 4]))
    }
}