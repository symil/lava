// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkHeadlessSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html).
///
/// Use the macro `VkHeadlessSurfaceCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkHeadlessSurfaceCreateFlags!()
/// ```
/// ```
/// VkHeadlessSurfaceCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkHeadlessSurfaceCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkHeadlessSurfaceCreateFlags = u32;

impl VkWrappedType<RawVkHeadlessSurfaceCreateFlags> for VkHeadlessSurfaceCreateFlags {
    fn vk_to_raw(src: &VkHeadlessSurfaceCreateFlags, dst: &mut RawVkHeadlessSurfaceCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkHeadlessSurfaceCreateFlags> for RawVkHeadlessSurfaceCreateFlags {
    fn vk_to_wrapped(src: &RawVkHeadlessSurfaceCreateFlags) -> VkHeadlessSurfaceCreateFlags {
        VkHeadlessSurfaceCreateFlags {
            
        }
    }
}

impl Default for VkHeadlessSurfaceCreateFlags {
    fn default() -> VkHeadlessSurfaceCreateFlags {
        VkHeadlessSurfaceCreateFlags {
            
        }
    }
}

impl VkHeadlessSurfaceCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkHeadlessSurfaceCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkHeadlessSurfaceCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkHeadlessSurfaceCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkHeadlessSurfaceCreateFlags {
    ( $( $x:ident ),* ) => {
        VkHeadlessSurfaceCreateFlags {
            $($x: true,)*
            ..VkHeadlessSurfaceCreateFlags::none()
        }
    }
}