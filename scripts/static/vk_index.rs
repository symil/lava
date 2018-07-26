use utils::vk_type::*;

impl VkRawType<usize> for u32 {
    fn vk_to_wrapped(value: &u32) -> usize {
        *value as usize
    }
}

impl VkWrappedType<u32> for usize {
    fn vk_to_raw(value: &usize, dst: &mut u32) {
        *dst = *value as u32;
    }
}

impl VkDefault for usize {
    fn vk_default() -> usize {
        0
    }
}