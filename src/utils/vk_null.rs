type VkNullType = [u8; 1024];

pub static VK_NULL_VALUE : VkNullType = [0; 1024];

pub fn vk_null_ref<T>() -> &'static T {
    unsafe {
        ((&VK_NULL_VALUE as *const VkNullType) as *const T).as_ref().unwrap()
    }
}