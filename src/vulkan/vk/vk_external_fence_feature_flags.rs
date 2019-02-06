// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalFenceFeatureFlagBits.html).
///
/// Use the macro `VkExternalFenceFeatureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkExternalFenceFeatureFlags!(exportable, importable)
/// ```
/// ```
/// VkExternalFenceFeatureFlags {
///     exportable: true,
///     importable: true,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkExternalFenceFeatureFlags {
    pub exportable: bool,
    pub importable: bool,
}

#[doc(hidden)]
pub type RawVkExternalFenceFeatureFlags = u32;

impl VkWrappedType<RawVkExternalFenceFeatureFlags> for VkExternalFenceFeatureFlags {
    fn vk_to_raw(src: &VkExternalFenceFeatureFlags, dst: &mut RawVkExternalFenceFeatureFlags) {
        *dst = 0;
        if src.exportable { *dst |= 0x00000001; }
        if src.importable { *dst |= 0x00000002; }
    }
}

impl VkRawType<VkExternalFenceFeatureFlags> for RawVkExternalFenceFeatureFlags {
    fn vk_to_wrapped(src: &RawVkExternalFenceFeatureFlags) -> VkExternalFenceFeatureFlags {
        VkExternalFenceFeatureFlags {
            exportable: (src & 0x00000001) != 0,
            importable: (src & 0x00000002) != 0,
        }
    }
}

impl Default for VkExternalFenceFeatureFlags {
    fn default() -> VkExternalFenceFeatureFlags {
        VkExternalFenceFeatureFlags {
            exportable: false,
            importable: false,
        }
    }
}

impl VkExternalFenceFeatureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkExternalFenceFeatureFlags {
            exportable: false,
            importable: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkExternalFenceFeatureFlags {
            exportable: true,
            importable: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.exportable { 0x00000001 } else { 0 }
        + if self.importable { 0x00000002 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkExternalFenceFeatureFlags {
            exportable: value & 0x00000001 > 0,
            importable: value & 0x00000002 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkExternalFenceFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkExternalFenceFeatureFlags {
            $($x: true,)*
            ..VkExternalFenceFeatureFlags::none()
        }
    }
}