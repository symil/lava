// Copied from `scripts/static/`

use utils::vk_traits::*;

/// Wrapper to specify a version number, that will be encoded according to the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/vkspec.html#fundamentals-versionnum). 
#[derive(Debug, Copy, Clone)]
pub struct VkVersion(pub u32, pub u32, pub u32);

impl VkRawType<VkVersion> for u32 {
    fn vk_to_wrapped(value: &u32) -> VkVersion {
        VkVersion(value >> 22, (value << 10) >> 22, (value << 20) >> 20)
    }
}

impl VkWrappedType<u32> for VkVersion {
    fn vk_to_raw(value: &VkVersion, dst: &mut u32) {
        *dst = ((value.0) << 22) | ((value.1) << 12) | (value.2);
    }
}

impl Default for VkVersion {
    fn default() -> VkVersion {
        VkVersion(0, 0, 0)
    }
}

impl VkVersion {
    pub fn one() -> VkVersion {
        VkVersion(1, 0, 0)
    }
}