impl VkType<u32> for usize {
    fn vk_to_raw(value: &usize, dst: &mut u32) {
        *dst = *value as u32;
    }

    fn vk_from_raw(value: &u32) -> usize {
        *value as usize
    }

    fn vk_default() -> usize {
        0
    }
}