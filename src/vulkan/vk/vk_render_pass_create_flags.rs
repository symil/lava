// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkRenderPassCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkRenderPassCreateFlagBits.html).
///
/// Use the macro `VkRenderPassCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkRenderPassCreateFlags!()
/// ```
/// ```
/// VkRenderPassCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkRenderPassCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkRenderPassCreateFlags = u32;

impl VkWrappedType<RawVkRenderPassCreateFlags> for VkRenderPassCreateFlags {
    fn vk_to_raw(src: &VkRenderPassCreateFlags, dst: &mut RawVkRenderPassCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkRenderPassCreateFlags> for RawVkRenderPassCreateFlags {
    fn vk_to_wrapped(src: &RawVkRenderPassCreateFlags) -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}

impl Default for VkRenderPassCreateFlags {
    fn default() -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}

impl VkRenderPassCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkRenderPassCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkRenderPassCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkRenderPassCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkRenderPassCreateFlags {
    ( $( $x:ident ),* ) => {
        VkRenderPassCreateFlags {
            $($x: true,)*
            ..VkRenderPassCreateFlags::none()
        }
    }
}