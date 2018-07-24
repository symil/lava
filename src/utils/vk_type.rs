pub trait VkType<R> {
    fn vk_to_raw(value: &Self, dst: &mut R);
    fn vk_from_raw(value: &R) -> Self;
    fn vk_default() -> Self;
}