use vk::*;

pub type VkHandler = usize;

#[repr(C)]
pub struct VecInfo<T> {
    pub ptr: *mut T,
    pub length: usize
}

#[repr(C)]
pub struct VkCreateHandlerResult {
    pub handler: VkHandler,
    pub code: VkResult
}