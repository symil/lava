// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalFenceFeatureFlagBits.html)
#[derive(Debug, Clone, Copy)]
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
    
    pub fn none() -> VkExternalFenceFeatureFlags {
        VkExternalFenceFeatureFlags {
            exportable: false,
            importable: false,
        }
    }
    
    pub fn all() -> VkExternalFenceFeatureFlags {
        VkExternalFenceFeatureFlags {
            exportable: true,
            importable: true,
        }
    }
}

#[macro_export]
macro_rules! VkExternalFenceFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkExternalFenceFeatureFlags {
            $($x: true,)*
            ..VkExternalFenceFeatureFlags::none()
        }
    }
}

impl VkExternalFenceFeatureFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.exportable { 0x00000001 } else { 0 }
        + if self.importable { 0x00000002 } else { 0 }
    }
}