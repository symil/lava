// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineDiscardRectangleStateCreateFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagBitsEXT.html).
///
/// Use the macro `VkPipelineDiscardRectangleStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineDiscardRectangleStateCreateFlags!()
/// ```
/// ```
/// VkPipelineDiscardRectangleStateCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkPipelineDiscardRectangleStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineDiscardRectangleStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineDiscardRectangleStateCreateFlags> for VkPipelineDiscardRectangleStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineDiscardRectangleStateCreateFlags, dst: &mut RawVkPipelineDiscardRectangleStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineDiscardRectangleStateCreateFlags> for RawVkPipelineDiscardRectangleStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineDiscardRectangleStateCreateFlags) -> VkPipelineDiscardRectangleStateCreateFlags {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineDiscardRectangleStateCreateFlags {
    fn default() -> VkPipelineDiscardRectangleStateCreateFlags {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
}

impl VkPipelineDiscardRectangleStateCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineDiscardRectangleStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineDiscardRectangleStateCreateFlags {
            $($x: true,)*
            ..VkPipelineDiscardRectangleStateCreateFlags::none()
        }
    }
}