// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDebugUtilsMessageTypeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html).
///
/// Use the macro `VkDebugUtilsMessageTypeFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDebugUtilsMessageTypeFlags!(general, validation)
/// ```
/// ```
/// VkDebugUtilsMessageTypeFlags {
///     general: true,
///     validation: true,
///     ..VkDebugUtilsMessageTypeFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkDebugUtilsMessageTypeFlags {
    pub general: bool,
    pub validation: bool,
    pub performance: bool,
}

#[doc(hidden)]
pub type RawVkDebugUtilsMessageTypeFlags = u32;

impl VkWrappedType<RawVkDebugUtilsMessageTypeFlags> for VkDebugUtilsMessageTypeFlags {
    fn vk_to_raw(src: &VkDebugUtilsMessageTypeFlags, dst: &mut RawVkDebugUtilsMessageTypeFlags) {
        *dst = 0;
        if src.general { *dst |= 0x00000001; }
        if src.validation { *dst |= 0x00000002; }
        if src.performance { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkDebugUtilsMessageTypeFlags> for RawVkDebugUtilsMessageTypeFlags {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessageTypeFlags) -> VkDebugUtilsMessageTypeFlags {
        VkDebugUtilsMessageTypeFlags {
            general: (src & 0x00000001) != 0,
            validation: (src & 0x00000002) != 0,
            performance: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkDebugUtilsMessageTypeFlags {
    fn default() -> VkDebugUtilsMessageTypeFlags {
        VkDebugUtilsMessageTypeFlags {
            general: false,
            validation: false,
            performance: false,
        }
    }
}

impl VkDebugUtilsMessageTypeFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDebugUtilsMessageTypeFlags {
            general: false,
            validation: false,
            performance: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDebugUtilsMessageTypeFlags {
            general: true,
            validation: true,
            performance: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.general { 0x00000001 } else { 0 }
        + if self.validation { 0x00000002 } else { 0 }
        + if self.performance { 0x00000004 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDebugUtilsMessageTypeFlags {
            general: value & 0x00000001 > 0,
            validation: value & 0x00000002 > 0,
            performance: value & 0x00000004 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDebugUtilsMessageTypeFlags {
    ( $( $x:ident ),* ) => {
        VkDebugUtilsMessageTypeFlags {
            $($x: true,)*
            ..VkDebugUtilsMessageTypeFlags::none()
        }
    }
}