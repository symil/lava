pub trait VkWrappedType<R> {
    fn vk_to_raw(value: &Self, dst: &mut R);
    fn vk_default() -> Self;
}

pub trait VkRawType<W> {
    fn vk_to_wrapped(src: &Self) -> W;
}