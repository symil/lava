#[derive(Debug)]
pub struct VkVersion(u32, u32, u32);

impl VkType<u32> for VkVersion {
    fn vk_to_raw(value: &Self, dst: &mut u32) {
        *dst = ((value.0) << 22) | ((value.1) << 12) | (value.2);
    }

    fn vk_from_raw(value: &u32) -> Self {
        VkVersion(value >> 22, (value << 10) >> 22, (value << 20) >> 20)
    }

    fn vk_default() -> Self {
        VkVersion(0, 0, 0)
    }
}

impl VkVersion {
    pub fn one() -> Self {
        Self(1, 0, 0)
    }
}