use utils::vk_type::*;

#[derive(Debug)]
pub struct VkVersion(u32, u32, u32);

impl VkRawType<VkVersion> for u32 {
    fn vk_to_wrapped(value: &u32) -> VkVersion {
        VkVersion(value >> 22, (value << 10) >> 22, (value << 20) >> 20)
    }
}

impl VkWrappedType<u32> for VkVersion {
    fn vk_to_raw(value: &VkVersion, dst: &mut u32) {
        *dst = ((value.0) << 22) | ((value.1) << 12) | (value.2);
    }

    fn vk_default() -> VkVersion {
        VkVersion(0, 0, 0)
    }
}

impl VkVersion {
    pub fn one() -> VkVersion {
        VkVersion(1, 0, 0)
    }
}