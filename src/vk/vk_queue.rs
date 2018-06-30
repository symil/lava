use std::convert::From;
use std::*;
use vk::*;
use libc::*;

pub struct VkQueue {
    _handle: RawVkQueue
}

impl VkQueue {
    pub fn get(device: &VkDevice, queue_family_index: usize, queue_index: usize) -> Self {
        unsafe {
            vk_call_retrieve_single_unchecked(|ptr| vkGetDeviceQueue(device.handle(), queue_family_index as u32, queue_index as u32, ptr), |x| {})
        }
    }
}

impl<'a> From<&'a RawVkQueue> for VkQueue {
    fn from(raw: &'a RawVkQueue) -> Self {
        Self {
            _handle: *raw
        }
    }
}