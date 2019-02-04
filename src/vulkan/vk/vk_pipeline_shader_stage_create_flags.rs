// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineShaderStageCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html)
///
/// Use the macro `VkPipelineShaderStageCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineShaderStageCreateFlags!()
/// ```
/// ```
/// VkPipelineShaderStageCreateFlags {
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineShaderStageCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineShaderStageCreateFlags = u32;

impl VkWrappedType<RawVkPipelineShaderStageCreateFlags> for VkPipelineShaderStageCreateFlags {
    fn vk_to_raw(src: &VkPipelineShaderStageCreateFlags, dst: &mut RawVkPipelineShaderStageCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineShaderStageCreateFlags> for RawVkPipelineShaderStageCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineShaderStageCreateFlags) -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

impl Default for VkPipelineShaderStageCreateFlags {
    fn default() -> VkPipelineShaderStageCreateFlags {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

impl VkPipelineShaderStageCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineShaderStageCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineShaderStageCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineShaderStageCreateFlags {
            $($x: true,)*
            ..VkPipelineShaderStageCreateFlags::none()
        }
    }
}