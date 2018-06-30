use std::*;
use vk::*;
use libc::*;

pub struct VkQueue {
    _handler: RawVkQueue
}

impl VkQueue {
    pub fn get(device: RawVkDevice, queue_family_index: usize, queue_index: usize) -> Self {
        unsafe {
            let mut queue_handler : RawVkQueue = 0;
            let queue_handler_ptr = &mut queue_handler as *mut RawVkQueue;

            vkGetDeviceQueue(device, queue_family_index as u32, queue_index as u32, queue_handler_ptr);

            VkQueue {
                _handler: queue_handler
            }
        }
    }
}