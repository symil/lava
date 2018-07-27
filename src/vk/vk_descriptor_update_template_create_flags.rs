// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkDescriptorUpdateTemplateCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
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

impl VkDefault for VkDescriptorUpdateTemplateCreateFlags {
    fn vk_default() -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}