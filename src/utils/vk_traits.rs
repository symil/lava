use vulkan::vk::RawVkInstance;
use vulkan::vk::RawVkDevice;
use vulkan::vk::VkFunctionTable;

pub trait VkWrappedType<R> {
    fn vk_to_raw(value: &Self, dst: &mut R);
}

pub trait VkRawType<W> {
    fn vk_to_wrapped(src: &Self) -> W;
}

pub trait VkSetup {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable);
}

pub trait VkFree {
    fn vk_free(&mut self);
}