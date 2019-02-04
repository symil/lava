// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkAttachmentDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAttachmentDescriptionFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentDescriptionFlags {
    pub may_alias: bool,
}

#[doc(hidden)]
pub type RawVkAttachmentDescriptionFlags = u32;

impl VkWrappedType<RawVkAttachmentDescriptionFlags> for VkAttachmentDescriptionFlags {
    fn vk_to_raw(src: &VkAttachmentDescriptionFlags, dst: &mut RawVkAttachmentDescriptionFlags) {
        *dst = 0;
        if src.may_alias { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkAttachmentDescriptionFlags> for RawVkAttachmentDescriptionFlags {
    fn vk_to_wrapped(src: &RawVkAttachmentDescriptionFlags) -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkAttachmentDescriptionFlags {
    fn default() -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: false,
        }
    }
}

impl VkAttachmentDescriptionFlags {
    
    pub fn none() -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: false,
        }
    }
    
    pub fn all() -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: true,
        }
    }
}

#[macro_export]
macro_rules! VkAttachmentDescriptionFlags {
    ( $( $x:ident ),* ) => {
        VkAttachmentDescriptionFlags {
            $($x: true,)*
            ..VkAttachmentDescriptionFlags::none()
        }
    }
}

impl VkAttachmentDescriptionFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.may_alias { 0x00000001 } else { 0 }
    }
    
    pub fn from_u32(value: u32) -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: value & 0x00000001 > 0,
        }
    }
}