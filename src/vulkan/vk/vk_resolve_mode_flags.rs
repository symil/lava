// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkResolveModeFlags](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkResolveModeFlags.html).
///
/// Use the macro `VkResolveModeFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkResolveModeFlags!(sample_zero, average)
/// ```
/// ```
/// VkResolveModeFlags {
///     sample_zero: true,
///     average: true,
///     ..VkResolveModeFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkResolveModeFlags {
    pub sample_zero: bool,
    pub average: bool,
    pub min: bool,
    pub max: bool,
}

#[doc(hidden)]
pub type RawVkResolveModeFlags = u32;

impl VkWrappedType<RawVkResolveModeFlags> for VkResolveModeFlags {
    fn vk_to_raw(src: &VkResolveModeFlags, dst: &mut RawVkResolveModeFlags) {
        *dst = 0;
        if src.sample_zero { *dst |= 0x00000001; }
        if src.average { *dst |= 0x00000002; }
        if src.min { *dst |= 0x00000004; }
        if src.max { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkResolveModeFlags> for RawVkResolveModeFlags {
    fn vk_to_wrapped(src: &RawVkResolveModeFlags) -> VkResolveModeFlags {
        VkResolveModeFlags {
            sample_zero: (src & 0x00000001) != 0,
            average: (src & 0x00000002) != 0,
            min: (src & 0x00000004) != 0,
            max: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkResolveModeFlags {
    fn default() -> VkResolveModeFlags {
        VkResolveModeFlags {
            sample_zero: false,
            average: false,
            min: false,
            max: false,
        }
    }
}

impl VkResolveModeFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkResolveModeFlags {
            sample_zero: false,
            average: false,
            min: false,
            max: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkResolveModeFlags {
            sample_zero: true,
            average: true,
            min: true,
            max: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.sample_zero { 0x00000001 } else { 0 }
        + if self.average { 0x00000002 } else { 0 }
        + if self.min { 0x00000004 } else { 0 }
        + if self.max { 0x00000008 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkResolveModeFlags {
            sample_zero: value & 0x00000001 > 0,
            average: value & 0x00000002 > 0,
            min: value & 0x00000004 > 0,
            max: value & 0x00000008 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkResolveModeFlags {
    ( $( $x:ident ),* ) => {
        VkResolveModeFlags {
            $($x: true,)*
            ..VkResolveModeFlags::none()
        }
    }
}