// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDisplayModeCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayModeCreateFlagBitsKHR.html)
#[derive(Debug, Clone, Copy)]
pub struct VkDisplayModeCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkDisplayModeCreateFlags = u32;

impl VkWrappedType<RawVkDisplayModeCreateFlags> for VkDisplayModeCreateFlags {
    fn vk_to_raw(src: &VkDisplayModeCreateFlags, dst: &mut RawVkDisplayModeCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDisplayModeCreateFlags> for RawVkDisplayModeCreateFlags {
    fn vk_to_wrapped(src: &RawVkDisplayModeCreateFlags) -> VkDisplayModeCreateFlags {
        VkDisplayModeCreateFlags {
            
        }
    }
}

impl Default for VkDisplayModeCreateFlags {
    fn default() -> VkDisplayModeCreateFlags {
        VkDisplayModeCreateFlags {
            
        }
    }
}

impl VkDisplayModeCreateFlags {
    
    pub fn none() -> VkDisplayModeCreateFlags {
        VkDisplayModeCreateFlags {
            
        }
    }
    
    pub fn all() -> VkDisplayModeCreateFlags {
        VkDisplayModeCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkDisplayModeCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDisplayModeCreateFlags {
            $($x: true,)*
            ..VkDisplayModeCreateFlags::none()
        }
    }
}

impl VkDisplayModeCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    pub fn from_u32(value: u32) -> VkDisplayModeCreateFlags {
        VkDisplayModeCreateFlags {
            
        }
    }
}