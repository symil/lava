// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSurfaceCounterFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCounterFlagsEXT.html).
///
/// Use the macro `VkSurfaceCounterFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkSurfaceCounterFlags!(vblank)
/// ```
/// ```
/// VkSurfaceCounterFlags {
///     vblank: true,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkSurfaceCounterFlags {
    pub vblank: bool,
}

#[doc(hidden)]
pub type RawVkSurfaceCounterFlags = u32;

impl VkWrappedType<RawVkSurfaceCounterFlags> for VkSurfaceCounterFlags {
    fn vk_to_raw(src: &VkSurfaceCounterFlags, dst: &mut RawVkSurfaceCounterFlags) {
        *dst = 0;
        if src.vblank { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkSurfaceCounterFlags> for RawVkSurfaceCounterFlags {
    fn vk_to_wrapped(src: &RawVkSurfaceCounterFlags) -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkSurfaceCounterFlags {
    fn default() -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: false,
        }
    }
}

impl VkSurfaceCounterFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkSurfaceCounterFlags {
            vblank: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkSurfaceCounterFlags {
            vblank: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.vblank { 0x00000001 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkSurfaceCounterFlags {
            vblank: value & 0x00000001 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkSurfaceCounterFlags {
    ( $( $x:ident ),* ) => {
        VkSurfaceCounterFlags {
            $($x: true,)*
            ..VkSurfaceCounterFlags::none()
        }
    }
}