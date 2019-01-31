// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineCoverageToColorStateCreateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagBitsNV.html)
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCoverageToColorStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineCoverageToColorStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineCoverageToColorStateCreateFlags> for VkPipelineCoverageToColorStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineCoverageToColorStateCreateFlags, dst: &mut RawVkPipelineCoverageToColorStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineCoverageToColorStateCreateFlags> for RawVkPipelineCoverageToColorStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCoverageToColorStateCreateFlags) -> VkPipelineCoverageToColorStateCreateFlags {
        VkPipelineCoverageToColorStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineCoverageToColorStateCreateFlags {
    fn default() -> VkPipelineCoverageToColorStateCreateFlags {
        VkPipelineCoverageToColorStateCreateFlags {
            
        }
    }
}

impl VkPipelineCoverageToColorStateCreateFlags {
    
    pub fn none() -> VkPipelineCoverageToColorStateCreateFlags {
        VkPipelineCoverageToColorStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineCoverageToColorStateCreateFlags {
        VkPipelineCoverageToColorStateCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkPipelineCoverageToColorStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineCoverageToColorStateCreateFlags {
            $($x: true,)*
            ..VkPipelineCoverageToColorStateCreateFlags::none()
        }
    }
}

impl VkPipelineCoverageToColorStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}