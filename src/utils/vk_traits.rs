use vk::RawVkInstance;
use vk::RawVkDevice;
use vk::VkInstanceFunctionTable;

pub trait VkWrappedType<R> {
    fn vk_to_raw(value: &Self, dst: &mut R);
}

pub trait VkRawType<W> {
    fn vk_to_wrapped(src: &Self) -> W;
}

pub trait VkSetup {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice);
}

pub trait VkFree {
    fn vk_free(&mut self);
}