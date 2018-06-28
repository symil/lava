use std::*;
use vk::*;
use wrapper::*;
use libc::*;

pub struct Queue {
    _handler: VkQueue
}

impl Queue {
    pub fn get(device: VkDevice, queue_family_index: usize, queue_index: usize) -> Self {
        unsafe {
            let mut queue_handler : VkQueue = 0;
            let queue_handler_ptr = &mut queue_handler as *mut VkQueue;

            vkGetDeviceQueue(device, queue_family_index as u32, queue_index as u32, queue_handler_ptr);

            Queue {
                _handler: queue_handler
            }
        }
    }
}