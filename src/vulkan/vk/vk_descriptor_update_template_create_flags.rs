// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDescriptorUpdateTemplateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html).
///
/// Use the macro `VkDescriptorUpdateTemplateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDescriptorUpdateTemplateCreateFlags!()
/// ```
/// ```
/// VkDescriptorUpdateTemplateCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
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
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDescriptorUpdateTemplateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDescriptorUpdateTemplateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDescriptorUpdateTemplateCreateFlags {
            $($x: true,)*
            ..VkDescriptorUpdateTemplateCreateFlags::none()
        }
    }
}