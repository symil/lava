struct VkNullType;
static VK_NULL_VALUE : VkNullType = VkNullType {};

pub fn vk_null<T>() -> &'static T {
    unsafe {
        ((&VK_NULL_VALUE as *const VkNullType) as *const T).as_ref().unwrap()
    }
}

pub fn vk_is_null<T>(value: &T) -> bool {
    value as *const T == (&VK_NULL_VALUE as *const VkNullType) as *const T
}