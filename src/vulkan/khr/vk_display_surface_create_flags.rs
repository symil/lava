// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDisplaySurfaceCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplaySurfaceCreateFlagBitsKHR.html)
#[derive(Debug, Clone, Copy)]
pub struct VkDisplaySurfaceCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkDisplaySurfaceCreateFlags = u32;

impl VkWrappedType<RawVkDisplaySurfaceCreateFlags> for VkDisplaySurfaceCreateFlags {
    fn vk_to_raw(src: &VkDisplaySurfaceCreateFlags, dst: &mut RawVkDisplaySurfaceCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDisplaySurfaceCreateFlags> for RawVkDisplaySurfaceCreateFlags {
    fn vk_to_wrapped(src: &RawVkDisplaySurfaceCreateFlags) -> VkDisplaySurfaceCreateFlags {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
}

impl Default for VkDisplaySurfaceCreateFlags {
    fn default() -> VkDisplaySurfaceCreateFlags {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
}

impl VkDisplaySurfaceCreateFlags {
    
    pub fn none() -> VkDisplaySurfaceCreateFlags {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
    
    pub fn all() -> VkDisplaySurfaceCreateFlags {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkDisplaySurfaceCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDisplaySurfaceCreateFlags {
            $($x: true,)*
            ..VkDisplaySurfaceCreateFlags::none()
        }
    }
}

impl VkDisplaySurfaceCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}