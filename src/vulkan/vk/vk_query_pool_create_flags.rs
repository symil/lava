// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkQueryPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPoolCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkQueryPoolCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkQueryPoolCreateFlags = u32;

impl VkWrappedType<RawVkQueryPoolCreateFlags> for VkQueryPoolCreateFlags {
    fn vk_to_raw(src: &VkQueryPoolCreateFlags, dst: &mut RawVkQueryPoolCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkQueryPoolCreateFlags> for RawVkQueryPoolCreateFlags {
    fn vk_to_wrapped(src: &RawVkQueryPoolCreateFlags) -> VkQueryPoolCreateFlags {
        VkQueryPoolCreateFlags {
            
        }
    }
}

impl Default for VkQueryPoolCreateFlags {
    fn default() -> VkQueryPoolCreateFlags {
        VkQueryPoolCreateFlags {
            
        }
    }
}

impl VkQueryPoolCreateFlags {
    
    pub fn none() -> VkQueryPoolCreateFlags {
        VkQueryPoolCreateFlags {
            
        }
    }
    
    pub fn all() -> VkQueryPoolCreateFlags {
        VkQueryPoolCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkQueryPoolCreateFlags {
    ( $( $x:ident ),* ) => {
        VkQueryPoolCreateFlags {
            $($x: true,)*
            ..VkQueryPoolCreateFlags::none()
        }
    }
}

impl VkQueryPoolCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}