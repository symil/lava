use std::*;
use vk::*;
use libc::*;

pub struct VkQueue {
    pub _handler: VkHandler
}

impl VkQueue {
    pub fn from_device(device: &VkDevice, queue_family: &VkQueueFamilyProperties, queue_index: usize) -> VkQueue {
        unsafe {
            let mut queue_handler : VkHandler = 0;
            let queue_handler_ptr = &mut queue_handler as *mut VkHandler;

            vkGetDeviceQueue(device._handler, queue_family._index as u32, queue_index as u32, queue_handler_ptr);

            VkQueue {
                _handler: queue_handler
            }
        }
    }
}