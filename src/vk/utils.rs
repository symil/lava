use std::*;
use std::vec::Vec;
use vk::VkResult;

pub type VkHandler = usize;

pub fn vk_make_version(version: &[u32; 3]) -> u32 {
    (((version[0]) << 22) | ((version[1]) << 12) | (version[2]))
}