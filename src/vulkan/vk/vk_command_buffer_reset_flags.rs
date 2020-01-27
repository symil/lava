// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCommandBufferResetFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferResetFlags.html).
///
/// Use the macro `VkCommandBufferResetFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkCommandBufferResetFlags!(release_resources)
/// ```
/// ```
/// VkCommandBufferResetFlags {
///     release_resources: true,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkCommandBufferResetFlags {
    pub release_resources: bool,
}

#[doc(hidden)]
pub type RawVkCommandBufferResetFlags = u32;

impl VkWrappedType<RawVkCommandBufferResetFlags> for VkCommandBufferResetFlags {
    fn vk_to_raw(src: &VkCommandBufferResetFlags, dst: &mut RawVkCommandBufferResetFlags) {
        *dst = 0;
        if src.release_resources { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkCommandBufferResetFlags> for RawVkCommandBufferResetFlags {
    fn vk_to_wrapped(src: &RawVkCommandBufferResetFlags) -> VkCommandBufferResetFlags {
        VkCommandBufferResetFlags {
            release_resources: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkCommandBufferResetFlags {
    fn default() -> VkCommandBufferResetFlags {
        VkCommandBufferResetFlags {
            release_resources: false,
        }
    }
}

impl VkCommandBufferResetFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkCommandBufferResetFlags {
            release_resources: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkCommandBufferResetFlags {
            release_resources: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.release_resources { 0x00000001 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkCommandBufferResetFlags {
            release_resources: value & 0x00000001 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkCommandBufferResetFlags {
    ( $( $x:ident ),* ) => {
        VkCommandBufferResetFlags {
            $($x: true,)*
            ..VkCommandBufferResetFlags::none()
        }
    }
}