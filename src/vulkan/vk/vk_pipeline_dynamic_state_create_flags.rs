// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineDynamicStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineDynamicStateCreateFlagBits.html)
///
/// Use the macro `VkPipelineDynamicStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineDynamicStateCreateFlags!()
/// ```
/// ```
/// VkPipelineDynamicStateCreateFlags {
/// }
/// ```
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
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineDynamicStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineDynamicStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineDynamicStateCreateFlags {
            $($x: true,)*
            ..VkPipelineDynamicStateCreateFlags::none()
        }
    }
}