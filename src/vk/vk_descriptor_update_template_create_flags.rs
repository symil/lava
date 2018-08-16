// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkDescriptorUpdateTemplateCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorUpdateTemplateCreateFlags {
    
}

impl VkRawType<VkDescriptorUpdateTemplateCreateFlags> for RawVkDescriptorUpdateTemplateCreateFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorUpdateTemplateCreateFlags) -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkDescriptorUpdateTemplateCreateFlags> for VkDescriptorUpdateTemplateCreateFlags {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplateCreateFlags, dst: &mut RawVkDescriptorUpdateTemplateCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkDescriptorUpdateTemplateCreateFlags {
    fn default() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}

impl VkDescriptorUpdateTemplateCreateFlags {
    
    pub fn none() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}