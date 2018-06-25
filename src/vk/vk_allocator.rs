use std::*;

#[repr(C)]
pub struct VkAllocator(u8);

impl VkAllocator {
    pub fn null() -> *const VkAllocator {
        ptr::null()
    }
}