// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkCullModeFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkCullModeFlags {
    front: bool,
    back: bool,
    front_and_back: bool,
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

impl VkWrappedType<RawVkCullModeFlags> for VkCullModeFlags {
    fn vk_to_raw(src: &VkCullModeFlags, dst: &mut RawVkCullModeFlags) {
        *dst = 0;
        if src.front { *dst |= 0x00000001; }
        if src.back { *dst |= 0x00000002; }
        if src.front_and_back { *dst |= 0x00000003; }
    }
}

impl VkDefault for VkCullModeFlags {
    fn vk_default() -> VkCullModeFlags {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
}