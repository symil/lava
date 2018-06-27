// Template generated by `scripts/generate_type.js`

use std::default::Default;
use std::convert::From;
use std::ops::Drop;
use std::ptr;
use libc::*;
use vk::*;

#[repr(C)]
pub struct RawVkBufferCreateInfo {
    s_type: VkStructureType,
    p_next: *const void,
    flags: RawVkBufferCreateFlags,
    size: u64,
    usage: RawVkBufferUsageFlags,
    sharing_mode: RawVkSharingMode,
    queue_family_index_count: u32,
    p_queue_family_indices: *mut u32,
}

pub struct VkBufferCreateInfo {
    pub flags: VkBufferCreateFlags,
    pub size: usize,
    pub usage: VkBufferUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_families: Vec<usize>
}

impl Default for VkBufferCreateInfo {
    fn default() -> Self {
        VkBufferCreateInfo {
            flags: VkBufferCreateFlags::none(),
            size: 0,
            usage: VkBufferUsageFlags::none(),
            sharing_mode: VkSharingMode::Exclusive,
            queue_families: Vec::new()
        }
    }
}

impl<'a> From<&'a VkBufferCreateInfo> for RawVkBufferCreateInfo {
    fn from(v: &'a VkBufferCreateInfo) -> Self {
        unsafe {
            RawVkBufferCreateInfo {
                s_type: VkStructureType::BufferCreateInfo,
                p_next: ptr::null(),
                flags: From::from(&v.flags),
                size: v.size as u64,
                usage: From::from(&v.usage),
                sharing_mode: From::from(&v.sharing_mode),
                queue_family_index_count: v.queue_families.len() as u32,
                p_queue_family_indices: copy_as_c_array(&v.queue_families.iter().map(|index| *index as u32).collect()),
            }
        }
    }
}

impl Drop for RawVkBufferCreateInfo {
    fn drop(&mut self) {
        unsafe {
            free_c_array(self.p_queue_family_indices);
        }
    }
}