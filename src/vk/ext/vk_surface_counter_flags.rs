// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSurfaceCounterFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCounterFlags {
    vblank: bool,
}

impl VkRawType<VkSurfaceCounterFlags> for RawVkSurfaceCounterFlags {
    fn vk_to_wrapped(src: &RawVkSurfaceCounterFlags) -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: (src & 0x00000001) != 0,
        }
    }
}

impl VkWrappedType<RawVkSurfaceCounterFlags> for VkSurfaceCounterFlags {
    fn vk_to_raw(src: &VkSurfaceCounterFlags, dst: &mut RawVkSurfaceCounterFlags) {
        *dst = 0;
        if src.vblank { *dst |= 0x00000001; }
    }
}

impl VkDefault for VkSurfaceCounterFlags {
    fn vk_default() -> VkSurfaceCounterFlags {
        VkSurfaceCounterFlags {
            vblank: false,
        }
    }
}