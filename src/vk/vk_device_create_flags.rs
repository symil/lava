// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;

pub type RawVkDeviceCreateFlags = u32;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct VkDeviceCreateFlags {
}

impl VkFlags for VkDeviceCreateFlags {
    
    fn none() -> Self {
        Self {
        }
    }
    
    fn all() -> Self {
        Self {
        }
    }
}

impl VkFrom<VkDeviceCreateFlags> for RawVkDeviceCreateFlags {
    
    fn vk_from(value: &VkDeviceCreateFlags) -> Self { {
            0
        }
    }
}

impl VkFrom<RawVkDeviceCreateFlags> for VkDeviceCreateFlags {
    
    fn vk_from(value: &RawVkDeviceCreateFlags) -> Self {
        Self {
        }
    }
}