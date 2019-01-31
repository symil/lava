// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineVertexInputStateCreateFlags {
    
}

pub type RawVkPipelineVertexInputStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineVertexInputStateCreateFlags> for VkPipelineVertexInputStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineVertexInputStateCreateFlags, dst: &mut RawVkPipelineVertexInputStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineVertexInputStateCreateFlags> for RawVkPipelineVertexInputStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineVertexInputStateCreateFlags) -> VkPipelineVertexInputStateCreateFlags {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineVertexInputStateCreateFlags {
    fn default() -> VkPipelineVertexInputStateCreateFlags {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
}

impl VkPipelineVertexInputStateCreateFlags {
    
    pub fn none() -> VkPipelineVertexInputStateCreateFlags {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineVertexInputStateCreateFlags {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkPipelineVertexInputStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineVertexInputStateCreateFlags {
            $($x: true,)*
            ..VkPipelineVertexInputStateCreateFlags::none()
        }
    }
}

impl VkPipelineVertexInputStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}