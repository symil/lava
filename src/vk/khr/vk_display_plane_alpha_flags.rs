// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkDisplayPlaneAlphaFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkDisplayPlaneAlphaFlags {
    pub opaque: bool,
    pub global: bool,
    pub per_pixel: bool,
    pub per_pixel_premultiplied: bool,
}

impl VkRawType<VkDisplayPlaneAlphaFlags> for RawVkDisplayPlaneAlphaFlags {
    fn vk_to_wrapped(src: &RawVkDisplayPlaneAlphaFlags) -> VkDisplayPlaneAlphaFlags {
        VkDisplayPlaneAlphaFlags {
            opaque: (src & 0x00000001) != 0,
            global: (src & 0x00000002) != 0,
            per_pixel: (src & 0x00000004) != 0,
            per_pixel_premultiplied: (src & 0x00000008) != 0,
        }
    }
}

impl VkWrappedType<RawVkDisplayPlaneAlphaFlags> for VkDisplayPlaneAlphaFlags {
    fn vk_to_raw(src: &VkDisplayPlaneAlphaFlags, dst: &mut RawVkDisplayPlaneAlphaFlags) {
        *dst = 0;
        if src.opaque { *dst |= 0x00000001; }
        if src.global { *dst |= 0x00000002; }
        if src.per_pixel { *dst |= 0x00000004; }
        if src.per_pixel_premultiplied { *dst |= 0x00000008; }
    }
}

impl Default for VkDisplayPlaneAlphaFlags {
    fn default() -> VkDisplayPlaneAlphaFlags {
        VkDisplayPlaneAlphaFlags {
            opaque: false,
            global: false,
            per_pixel: false,
            per_pixel_premultiplied: false,
        }
    }
}

impl VkDisplayPlaneAlphaFlags {
    
    pub fn none() -> VkDisplayPlaneAlphaFlags {
        VkDisplayPlaneAlphaFlags {
            opaque: false,
            global: false,
            per_pixel: false,
            per_pixel_premultiplied: false,
        }
    }
    
    pub fn all() -> VkDisplayPlaneAlphaFlags {
        VkDisplayPlaneAlphaFlags {
            opaque: true,
            global: true,
            per_pixel: true,
            per_pixel_premultiplied: true,
        }
    }
}