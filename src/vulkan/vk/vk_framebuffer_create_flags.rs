// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkFramebufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFramebufferCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkFramebufferCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkFramebufferCreateFlags = u32;

impl VkWrappedType<RawVkFramebufferCreateFlags> for VkFramebufferCreateFlags {
    fn vk_to_raw(src: &VkFramebufferCreateFlags, dst: &mut RawVkFramebufferCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkFramebufferCreateFlags> for RawVkFramebufferCreateFlags {
    fn vk_to_wrapped(src: &RawVkFramebufferCreateFlags) -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
}

impl Default for VkFramebufferCreateFlags {
    fn default() -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
}

impl VkFramebufferCreateFlags {
    
    pub fn none() -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
    
    pub fn all() -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkFramebufferCreateFlags {
    ( $( $x:ident ),* ) => {
        VkFramebufferCreateFlags {
            $($x: true,)*
            ..VkFramebufferCreateFlags::none()
        }
    }
}

impl VkFramebufferCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    pub fn from_u32(value: u32) -> VkFramebufferCreateFlags {
        VkFramebufferCreateFlags {
            
        }
    }
}