// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineTessellationStateCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineTessellationStateCreateFlags {
    
}

impl VkRawType<VkPipelineTessellationStateCreateFlags> for RawVkPipelineTessellationStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineTessellationStateCreateFlags) -> VkPipelineTessellationStateCreateFlags {
        VkPipelineTessellationStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineTessellationStateCreateFlags> for VkPipelineTessellationStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineTessellationStateCreateFlags, dst: &mut RawVkPipelineTessellationStateCreateFlags) {
        *dst = 0;
    }
}

impl VkDefault for VkPipelineTessellationStateCreateFlags {
    fn vk_default() -> VkPipelineTessellationStateCreateFlags {
        VkPipelineTessellationStateCreateFlags {
            
        }
    }
}