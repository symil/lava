use std::vec::Vec;
use std::default::Default;
use std::convert::From;
use std::ops::Drop;
use std::ptr;
use libc::*;
use vk::*;

#[repr(C)]
pub struct RawVkDeviceQueueCreateInfo {
    s_type: VkStructureType,
    p_next: *const u8,
    flags: RawVkDeviceQueueCreateFlags,
    queue_family_index: u32,
    queue_count: u32,
    p_queue_priorities: *mut f32
}

pub struct VkDeviceQueueCreateInfo {
    pub flags: VkDeviceQueueCreateFlags,
    pub queue_family_index: usize,
    pub queue_priorities: Vec<f32>
}

impl Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        VkDeviceQueueCreateInfo {
            flags: VkDeviceQueueCreateFlags {
                protected: false
            },
            queue_family_index: 0,
            queue_priorities: vec![1.0]
        }
    }
}

impl<'a> From<&'a VkDeviceQueueCreateInfo> for RawVkDeviceQueueCreateInfo {
    fn from(v: &'a VkDeviceQueueCreateInfo) -> Self {
        unsafe {
            RawVkDeviceQueueCreateInfo {
                s_type: VkStructureType::DeviceQueueCreateInfo,
                p_next: ptr::null(),
                flags: RawVkDeviceQueueCreateFlags::from(&v.flags),
                queue_family_index: v.queue_family_index as u32,
                queue_count: v.queue_priorities.len() as u32,
                p_queue_priorities: copy_as_c_array(&v.queue_priorities)
            }
        }
    }
}

impl Drop for RawVkDeviceQueueCreateInfo {
    fn drop(&mut self) {
        unsafe {
            free_c_array(self.p_queue_priorities)
        }
    }
}