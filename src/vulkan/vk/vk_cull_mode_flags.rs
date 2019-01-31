// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCullModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCullModeFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkCullModeFlags {
    pub front: bool,
    pub back: bool,
    pub front_and_back: bool,
}

#[doc(hidden)]
pub type RawVkCullModeFlags = u32;

impl VkWrappedType<RawVkCullModeFlags> for VkCullModeFlags {
    fn vk_to_raw(src: &VkCullModeFlags, dst: &mut RawVkCullModeFlags) {
        *dst = 0;
        if src.front { *dst |= 0x00000001; }
        if src.back { *dst |= 0x00000002; }
        if src.front_and_back { *dst |= 0x00000003; }
    }
}

impl VkRawType<VkCullModeFlags> for RawVkCullModeFlags {
    fn vk_to_wrapped(src: &RawVkCullModeFlags) -> VkCullModeFlags {
        VkCullModeFlags {
            front: (src & 0x00000001) != 0,
            back: (src & 0x00000002) != 0,
            front_and_back: (src & 0x00000003) != 0,
        }
    }
}

impl Default for VkCullModeFlags {
    fn default() -> VkCullModeFlags {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
}

impl VkCullModeFlags {
    
    pub fn none() -> VkCullModeFlags {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
    
    pub fn all() -> VkCullModeFlags {
        VkCullModeFlags {
            front: true,
            back: true,
            front_and_back: true,
        }
    }
}

#[macro_export]
macro_rules! VkCullModeFlags {
    ( $( $x:ident ),* ) => {
        VkCullModeFlags {
            $($x: true,)*
            ..VkCullModeFlags::none()
        }
    }
}

impl VkCullModeFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.front { 0x00000001 } else { 0 }
        + if self.back { 0x00000002 } else { 0 }
        + if self.front_and_back { 0x00000003 } else { 0 }
    }
}