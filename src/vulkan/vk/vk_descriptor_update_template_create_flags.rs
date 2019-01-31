// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDescriptorUpdateTemplateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorUpdateTemplateCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorUpdateTemplateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkDescriptorUpdateTemplateCreateFlags = u32;

impl VkWrappedType<RawVkDescriptorUpdateTemplateCreateFlags> for VkDescriptorUpdateTemplateCreateFlags {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplateCreateFlags, dst: &mut RawVkDescriptorUpdateTemplateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDescriptorUpdateTemplateCreateFlags> for RawVkDescriptorUpdateTemplateCreateFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorUpdateTemplateCreateFlags) -> VkDescriptorUpdateTemplateCreateFlags {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
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

#[macro_export]
macro_rules! VkDescriptorUpdateTemplateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDescriptorUpdateTemplateCreateFlags {
            $($x: true,)*
            ..VkDescriptorUpdateTemplateCreateFlags::none()
        }
    }
}

impl VkDescriptorUpdateTemplateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}