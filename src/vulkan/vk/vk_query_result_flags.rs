// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkQueryResultFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryResultFlagBits.html)
///
/// Use the macro `VkQueryResultFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkQueryResultFlags!(_64, wait)
/// ```
/// ```
/// VkQueryResultFlags {
///     _64: true,
///     wait: true,
///     ..VkQueryResultFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkQueryResultFlags {
    pub _64: bool,
    pub wait: bool,
    pub with_availability: bool,
    pub partial: bool,
}

#[doc(hidden)]
pub type RawVkQueryResultFlags = u32;

impl VkWrappedType<RawVkQueryResultFlags> for VkQueryResultFlags {
    fn vk_to_raw(src: &VkQueryResultFlags, dst: &mut RawVkQueryResultFlags) {
        *dst = 0;
        if src._64 { *dst |= 0x00000001; }
        if src.wait { *dst |= 0x00000002; }
        if src.with_availability { *dst |= 0x00000004; }
        if src.partial { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkQueryResultFlags> for RawVkQueryResultFlags {
    fn vk_to_wrapped(src: &RawVkQueryResultFlags) -> VkQueryResultFlags {
        VkQueryResultFlags {
            _64: (src & 0x00000001) != 0,
            wait: (src & 0x00000002) != 0,
            with_availability: (src & 0x00000004) != 0,
            partial: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkQueryResultFlags {
    fn default() -> VkQueryResultFlags {
        VkQueryResultFlags {
            _64: false,
            wait: false,
            with_availability: false,
            partial: false,
        }
    }
}

impl VkQueryResultFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkQueryResultFlags {
            _64: false,
            wait: false,
            with_availability: false,
            partial: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkQueryResultFlags {
            _64: true,
            wait: true,
            with_availability: true,
            partial: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self._64 { 0x00000001 } else { 0 }
        + if self.wait { 0x00000002 } else { 0 }
        + if self.with_availability { 0x00000004 } else { 0 }
        + if self.partial { 0x00000008 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkQueryResultFlags {
            _64: value & 0x00000001 > 0,
            wait: value & 0x00000002 > 0,
            with_availability: value & 0x00000004 > 0,
            partial: value & 0x00000008 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkQueryResultFlags {
    ( $( $x:ident ),* ) => {
        VkQueryResultFlags {
            $($x: true,)*
            ..VkQueryResultFlags::none()
        }
    }
}