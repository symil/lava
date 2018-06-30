// Generated by `scripts/generate_type.js`

use std::convert::From;

pub type RawVkSwapchainCreateFlags = u32;

#[derive(Debug, Default, Copy, Clone)]
pub struct VkSwapchainCreateFlags {
    pub split_instance_bind_regions: bool,
    pub protected: bool
}

impl<'a> From<&'a u32> for VkSwapchainCreateFlags {
    fn from(value: &'a u32) -> Self {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: (value & 0x00000001) > 0,
            protected: (value & 0x00000002) > 0
        }
    }
}

impl<'a> From<&'a VkSwapchainCreateFlags> for u32 {
    fn from(value: &'a VkSwapchainCreateFlags) -> Self {
        (if value.split_instance_bind_regions { 0x00000001 } else { 0 }) +
        (if value.protected { 0x00000002 } else { 0 })
    }
}

impl VkSwapchainCreateFlags {
    pub fn none() -> Self {
        VkSwapchainCreateFlags::default()
    }

    pub fn all() -> Self {
        VkSwapchainCreateFlags {
            split_instance_bind_regions: true,
            protected: true
        }
    }
}