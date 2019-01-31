// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkRenderPassCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkRenderPassCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkRenderPassCreateFlags = u32;

impl VkWrappedType<RawVkRenderPassCreateFlags> for VkRenderPassCreateFlags {
    fn vk_to_raw(src: &VkRenderPassCreateFlags, dst: &mut RawVkRenderPassCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkRenderPassCreateFlags> for RawVkRenderPassCreateFlags {
    fn vk_to_wrapped(src: &RawVkRenderPassCreateFlags) -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}

impl Default for VkRenderPassCreateFlags {
    fn default() -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}

impl VkRenderPassCreateFlags {
    
    pub fn none() -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
    
    pub fn all() -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkRenderPassCreateFlags {
    ( $( $x:ident ),* ) => {
        VkRenderPassCreateFlags {
            $($x: true,)*
            ..VkRenderPassCreateFlags::none()
        }
    }
}

impl VkRenderPassCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}