// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineViewportStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineViewportStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineViewportStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineViewportStateCreateFlags> for VkPipelineViewportStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineViewportStateCreateFlags, dst: &mut RawVkPipelineViewportStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineViewportStateCreateFlags> for RawVkPipelineViewportStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineViewportStateCreateFlags) -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineViewportStateCreateFlags {
    fn default() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

impl VkPipelineViewportStateCreateFlags {
    
    pub fn none() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkPipelineViewportStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineViewportStateCreateFlags {
            $($x: true,)*
            ..VkPipelineViewportStateCreateFlags::none()
        }
    }
}

impl VkPipelineViewportStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    pub fn from_u32(value: u32) -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}