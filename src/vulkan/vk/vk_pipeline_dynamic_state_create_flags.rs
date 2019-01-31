// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineDynamicStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineDynamicStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineDynamicStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineDynamicStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineDynamicStateCreateFlags> for VkPipelineDynamicStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineDynamicStateCreateFlags, dst: &mut RawVkPipelineDynamicStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineDynamicStateCreateFlags> for RawVkPipelineDynamicStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineDynamicStateCreateFlags) -> VkPipelineDynamicStateCreateFlags {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineDynamicStateCreateFlags {
    fn default() -> VkPipelineDynamicStateCreateFlags {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
}

impl VkPipelineDynamicStateCreateFlags {
    
    pub fn none() -> VkPipelineDynamicStateCreateFlags {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineDynamicStateCreateFlags {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkPipelineDynamicStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineDynamicStateCreateFlags {
            $($x: true,)*
            ..VkPipelineDynamicStateCreateFlags::none()
        }
    }
}

impl VkPipelineDynamicStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}