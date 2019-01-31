// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDeviceQueueCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceQueueCreateFlags {
    pub protected: bool,
}

#[doc(hidden)]
pub type RawVkDeviceQueueCreateFlags = u32;

impl VkWrappedType<RawVkDeviceQueueCreateFlags> for VkDeviceQueueCreateFlags {
    fn vk_to_raw(src: &VkDeviceQueueCreateFlags, dst: &mut RawVkDeviceQueueCreateFlags) {
        *dst = 0;
        if src.protected { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkDeviceQueueCreateFlags> for RawVkDeviceQueueCreateFlags {
    fn vk_to_wrapped(src: &RawVkDeviceQueueCreateFlags) -> VkDeviceQueueCreateFlags {
        VkDeviceQueueCreateFlags {
            protected: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkDeviceQueueCreateFlags {
    fn default() -> VkDeviceQueueCreateFlags {
        VkDeviceQueueCreateFlags {
            protected: false,
        }
    }
}

impl VkDeviceQueueCreateFlags {
    
    pub fn none() -> VkDeviceQueueCreateFlags {
        VkDeviceQueueCreateFlags {
            protected: false,
        }
    }
    
    pub fn all() -> VkDeviceQueueCreateFlags {
        VkDeviceQueueCreateFlags {
            protected: true,
        }
    }
}

#[macro_export]
macro_rules! VkDeviceQueueCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDeviceQueueCreateFlags {
            $($x: true,)*
            ..VkDeviceQueueCreateFlags::none()
        }
    }
}

impl VkDeviceQueueCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.protected { 0x00000001 } else { 0 }
    }
}