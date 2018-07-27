// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkImageCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkImageCreateFlags {
    sparse_binding: bool,
    sparse_residency: bool,
    sparse_aliased: bool,
    mutable_format: bool,
    cube_compatible: bool,
    alias: bool,
    split_instance_bind_regions: bool,
    _2d_array_compatible: bool,
    block_texel_view_compatible: bool,
    extended_usage: bool,
    protected: bool,
    disjoint: bool,
    sample_locations_compatible_depth_ext: bool,
}

impl VkRawType<VkImageCreateFlags> for RawVkImageCreateFlags {
    fn vk_to_wrapped(src: &RawVkImageCreateFlags) -> VkImageCreateFlags {
        VkImageCreateFlags {
            sparse_binding: (src & 0x00000001) != 0,
            sparse_residency: (src & 0x00000002) != 0,
            sparse_aliased: (src & 0x00000004) != 0,
            mutable_format: (src & 0x00000008) != 0,
            cube_compatible: (src & 0x00000010) != 0,
            alias: (src & 0x00000400) != 0,
            split_instance_bind_regions: (src & 0x00000040) != 0,
            _2d_array_compatible: (src & 0x00000020) != 0,
            block_texel_view_compatible: (src & 0x00000080) != 0,
            extended_usage: (src & 0x00000100) != 0,
            protected: (src & 0x00000800) != 0,
            disjoint: (src & 0x00000200) != 0,
            sample_locations_compatible_depth_ext: (src & 0x00001000) != 0,
        }
    }
}

impl VkWrappedType<RawVkImageCreateFlags> for VkImageCreateFlags {
    fn vk_to_raw(src: &VkImageCreateFlags, dst: &mut RawVkImageCreateFlags) {
        *dst = 0;
        if src.sparse_binding { *dst |= 0x00000001; }
        if src.sparse_residency { *dst |= 0x00000002; }
        if src.sparse_aliased { *dst |= 0x00000004; }
        if src.mutable_format { *dst |= 0x00000008; }
        if src.cube_compatible { *dst |= 0x00000010; }
        if src.alias { *dst |= 0x00000400; }
        if src.split_instance_bind_regions { *dst |= 0x00000040; }
        if src._2d_array_compatible { *dst |= 0x00000020; }
        if src.block_texel_view_compatible { *dst |= 0x00000080; }
        if src.extended_usage { *dst |= 0x00000100; }
        if src.protected { *dst |= 0x00000800; }
        if src.disjoint { *dst |= 0x00000200; }
        if src.sample_locations_compatible_depth_ext { *dst |= 0x00001000; }
    }
}

impl VkDefault for VkImageCreateFlags {
    fn vk_default() -> VkImageCreateFlags {
        VkImageCreateFlags {
            sparse_binding: false,
            sparse_residency: false,
            sparse_aliased: false,
            mutable_format: false,
            cube_compatible: false,
            alias: false,
            split_instance_bind_regions: false,
            _2d_array_compatible: false,
            block_texel_view_compatible: false,
            extended_usage: false,
            protected: false,
            disjoint: false,
            sample_locations_compatible_depth_ext: false,
        }
    }
}