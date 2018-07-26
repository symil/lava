pub trait VkWrappedType<R> {
    fn vk_to_raw(value: &Self, dst: &mut R);
}

pub trait VkRawType<W> {
    fn vk_to_wrapped(src: &Self) -> W;
}

pub trait VkDefault {
    fn vk_default() -> Self;
}