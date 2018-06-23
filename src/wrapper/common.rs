pub type VkHandler = usize;

#[repr(C)]
pub struct VecInfo<T> {
    ptr: *mut T,
    length: usize
}

#[repr(C)]
pub struct VkCreateHandlerResult {
    handler: VkHandler,
    code: VkResult
}