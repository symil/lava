// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkShaderStageFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkShaderStageFlags {
    vertex: bool,
    tessellation_control: bool,
    tessellation_evaluation: bool,
    geometry: bool,
    fragment: bool,
    compute: bool,
    all_graphics: bool,
}

impl VkRawType<VkShaderStageFlags> for RawVkShaderStageFlags {
    fn vk_to_wrapped(src: &RawVkShaderStageFlags) -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: (src & 0x00000001) != 0,
            tessellation_control: (src & 0x00000002) != 0,
            tessellation_evaluation: (src & 0x00000004) != 0,
            geometry: (src & 0x00000008) != 0,
            fragment: (src & 0x00000010) != 0,
            compute: (src & 0x00000020) != 0,
            all_graphics: (src & 0x0000001F) != 0,
        }
    }
}

impl VkWrappedType<RawVkShaderStageFlags> for VkShaderStageFlags {
    fn vk_to_raw(src: &VkShaderStageFlags, dst: &mut RawVkShaderStageFlags) {
        *dst = 0;
        if src.vertex { *dst |= 0x00000001; }
        if src.tessellation_control { *dst |= 0x00000002; }
        if src.tessellation_evaluation { *dst |= 0x00000004; }
        if src.geometry { *dst |= 0x00000008; }
        if src.fragment { *dst |= 0x00000010; }
        if src.compute { *dst |= 0x00000020; }
        if src.all_graphics { *dst |= 0x0000001F; }
    }
}

impl VkDefault for VkShaderStageFlags {
    fn vk_default() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: false,
            tessellation_control: false,
            tessellation_evaluation: false,
            geometry: false,
            fragment: false,
            compute: false,
            all_graphics: false,
        }
    }
}