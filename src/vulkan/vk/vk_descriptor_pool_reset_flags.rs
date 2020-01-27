// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDescriptorPoolResetFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolResetFlags.html).
///
/// Use the macro `VkDescriptorPoolResetFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDescriptorPoolResetFlags!()
/// ```
/// ```
/// VkDescriptorPoolResetFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkDescriptorPoolResetFlags {
    
}

#[doc(hidden)]
pub type RawVkDescriptorPoolResetFlags = u32;

impl VkWrappedType<RawVkDescriptorPoolResetFlags> for VkDescriptorPoolResetFlags {
    fn vk_to_raw(src: &VkDescriptorPoolResetFlags, dst: &mut RawVkDescriptorPoolResetFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDescriptorPoolResetFlags> for RawVkDescriptorPoolResetFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorPoolResetFlags) -> VkDescriptorPoolResetFlags {
        VkDescriptorPoolResetFlags {
            
        }
    }
}

impl Default for VkDescriptorPoolResetFlags {
    fn default() -> VkDescriptorPoolResetFlags {
        VkDescriptorPoolResetFlags {
            
        }
    }
}

impl VkDescriptorPoolResetFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDescriptorPoolResetFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDescriptorPoolResetFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDescriptorPoolResetFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDescriptorPoolResetFlags {
    ( $( $x:ident ),* ) => {
        VkDescriptorPoolResetFlags {
            $($x: true,)*
            ..VkDescriptorPoolResetFlags::none()
        }
    }
}