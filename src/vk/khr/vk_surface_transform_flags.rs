// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSurfaceTransformFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceTransformFlags {
    identity: bool,
    rotate_90: bool,
    rotate_180: bool,
    rotate_270: bool,
    horizontal_mirror: bool,
    horizontal_mirror_rotate_90: bool,
    horizontal_mirror_rotate_180: bool,
    horizontal_mirror_rotate_270: bool,
    inherit: bool,
}

impl VkRawType<VkSurfaceTransformFlags> for RawVkSurfaceTransformFlags {
    fn vk_to_wrapped(src: &RawVkSurfaceTransformFlags) -> VkSurfaceTransformFlags {
        VkSurfaceTransformFlags {
            identity: (src & 0x00000001) != 0,
            rotate_90: (src & 0x00000002) != 0,
            rotate_180: (src & 0x00000004) != 0,
            rotate_270: (src & 0x00000008) != 0,
            horizontal_mirror: (src & 0x00000010) != 0,
            horizontal_mirror_rotate_90: (src & 0x00000020) != 0,
            horizontal_mirror_rotate_180: (src & 0x00000040) != 0,
            horizontal_mirror_rotate_270: (src & 0x00000080) != 0,
            inherit: (src & 0x00000100) != 0,
        }
    }
}

impl VkWrappedType<RawVkSurfaceTransformFlags> for VkSurfaceTransformFlags {
    fn vk_to_raw(src: &VkSurfaceTransformFlags, dst: &mut RawVkSurfaceTransformFlags) {
        *dst = 0;
        if src.identity { *dst |= 0x00000001; }
        if src.rotate_90 { *dst |= 0x00000002; }
        if src.rotate_180 { *dst |= 0x00000004; }
        if src.rotate_270 { *dst |= 0x00000008; }
        if src.horizontal_mirror { *dst |= 0x00000010; }
        if src.horizontal_mirror_rotate_90 { *dst |= 0x00000020; }
        if src.horizontal_mirror_rotate_180 { *dst |= 0x00000040; }
        if src.horizontal_mirror_rotate_270 { *dst |= 0x00000080; }
        if src.inherit { *dst |= 0x00000100; }
    }
}

pub static STATIC_VK_SURFACE_TRANSFORM_FLAGS : VkSurfaceTransformFlags = VkSurfaceTransformFlags {
    identity: false,
    rotate_90: false,
    rotate_180: false,
    rotate_270: false,
    horizontal_mirror: false,
    horizontal_mirror_rotate_90: false,
    horizontal_mirror_rotate_180: false,
    horizontal_mirror_rotate_270: false,
    inherit: false,
};

impl VkDefault for VkSurfaceTransformFlags {
    fn vk_default() -> VkSurfaceTransformFlags {
        STATIC_VK_SURFACE_TRANSFORM_FLAGS
    }
}