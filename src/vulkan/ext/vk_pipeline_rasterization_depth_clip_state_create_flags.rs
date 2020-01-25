// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineRasterizationDepthClipStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html).
///
/// Use the macro `VkPipelineRasterizationDepthClipStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineRasterizationDepthClipStateCreateFlags!()
/// ```
/// ```
/// VkPipelineRasterizationDepthClipStateCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationDepthClipStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineRasterizationDepthClipStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineRasterizationDepthClipStateCreateFlags> for VkPipelineRasterizationDepthClipStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineRasterizationDepthClipStateCreateFlags, dst: &mut RawVkPipelineRasterizationDepthClipStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineRasterizationDepthClipStateCreateFlags> for RawVkPipelineRasterizationDepthClipStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationDepthClipStateCreateFlags) -> VkPipelineRasterizationDepthClipStateCreateFlags {
        VkPipelineRasterizationDepthClipStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineRasterizationDepthClipStateCreateFlags {
    fn default() -> VkPipelineRasterizationDepthClipStateCreateFlags {
        VkPipelineRasterizationDepthClipStateCreateFlags {
            
        }
    }
}

impl VkPipelineRasterizationDepthClipStateCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineRasterizationDepthClipStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineRasterizationDepthClipStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineRasterizationDepthClipStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineRasterizationDepthClipStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineRasterizationDepthClipStateCreateFlags {
            $($x: true,)*
            ..VkPipelineRasterizationDepthClipStateCreateFlags::none()
        }
    }
}