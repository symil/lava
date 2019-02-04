// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineViewportStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineViewportStateCreateFlagBits.html)
///
/// Use the macro `VkPipelineViewportStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineViewportStateCreateFlags!()
/// ```
/// ```
/// VkPipelineViewportStateCreateFlags {
/// }
/// ```
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
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineViewportStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineViewportStateCreateFlags {
            $($x: true,)*
            ..VkPipelineViewportStateCreateFlags::none()
        }
    }
}