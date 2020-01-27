// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkIndexType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndexType.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkIndexType {
    Uint16 = 0,
    Uint32 = 1,
    NoneNv = 1000165000,
    Uint8Ext = 1000265000,
}

#[doc(hidden)]
pub type RawVkIndexType = i32;

impl VkWrappedType<RawVkIndexType> for VkIndexType {
    fn vk_to_raw(src: &VkIndexType, dst: &mut RawVkIndexType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkIndexType> for RawVkIndexType {
    fn vk_to_wrapped(src: &RawVkIndexType) -> VkIndexType {
        unsafe {
            *((src as *const i32) as *const VkIndexType)
        }
    }
}

impl Default for VkIndexType {
    fn default() -> VkIndexType {
        VkIndexType::Uint16
    }
}