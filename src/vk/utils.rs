use std::*;
use std::vec::Vec;
use vk::VkResult;

pub type VkHandler = usize;

#[repr(C)]
pub struct VecInfo<T> {
    pub ptr: *mut T,
    pub length: usize
}

impl<T> VecInfo<T> {
    pub fn to_vec(&self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(self.ptr, self.length, self.length) }
    }
}

#[repr(C)]
pub struct VkCreateHandlerResult {
    pub handler: VkHandler,
    pub code: VkResult
}

pub fn vk_make_version(version: &[u32; 3]) -> u32 {
    (((version[0]) << 22) | ((version[1]) << 12) | (version[2]))
}