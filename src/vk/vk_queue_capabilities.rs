use std::*;

#[derive(Debug)]
pub struct VkQueueCapabilities {
    pub graphics: bool,
    pub compute: bool,
    pub transfer: bool,
    pub sparse_binding: bool,
    pub protected: bool
}

impl convert::From<u32> for VkQueueCapabilities {
    fn from(value: u32) -> Self {
        VkQueueCapabilities {
            graphics: (value & 0x00000001) > 0,
            compute: (value & 0x00000002) > 0,
            transfer: (value & 0x00000004) > 0,
            sparse_binding: (value & 0x00000008) > 0,
            protected: (value & 0x00000010) > 0
        }
    }
}