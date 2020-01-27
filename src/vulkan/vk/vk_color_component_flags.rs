// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkColorComponentFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorComponentFlags.html).
///
/// Use the macro `VkColorComponentFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkColorComponentFlags!(r, g)
/// ```
/// ```
/// VkColorComponentFlags {
///     r: true,
///     g: true,
///     ..VkColorComponentFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkColorComponentFlags {
    pub r: bool,
    pub g: bool,
    pub b: bool,
    pub a: bool,
}

#[doc(hidden)]
pub type RawVkColorComponentFlags = u32;

impl VkWrappedType<RawVkColorComponentFlags> for VkColorComponentFlags {
    fn vk_to_raw(src: &VkColorComponentFlags, dst: &mut RawVkColorComponentFlags) {
        *dst = 0;
        if src.r { *dst |= 0x00000001; }
        if src.g { *dst |= 0x00000002; }
        if src.b { *dst |= 0x00000004; }
        if src.a { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkColorComponentFlags> for RawVkColorComponentFlags {
    fn vk_to_wrapped(src: &RawVkColorComponentFlags) -> VkColorComponentFlags {
        VkColorComponentFlags {
            r: (src & 0x00000001) != 0,
            g: (src & 0x00000002) != 0,
            b: (src & 0x00000004) != 0,
            a: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkColorComponentFlags {
    fn default() -> VkColorComponentFlags {
        VkColorComponentFlags {
            r: false,
            g: false,
            b: false,
            a: false,
        }
    }
}

impl VkColorComponentFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkColorComponentFlags {
            r: false,
            g: false,
            b: false,
            a: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkColorComponentFlags {
            r: true,
            g: true,
            b: true,
            a: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.r { 0x00000001 } else { 0 }
        + if self.g { 0x00000002 } else { 0 }
        + if self.b { 0x00000004 } else { 0 }
        + if self.a { 0x00000008 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkColorComponentFlags {
            r: value & 0x00000001 > 0,
            g: value & 0x00000002 > 0,
            b: value & 0x00000004 > 0,
            a: value & 0x00000008 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkColorComponentFlags {
    ( $( $x:ident ),* ) => {
        VkColorComponentFlags {
            $($x: true,)*
            ..VkColorComponentFlags::none()
        }
    }
}