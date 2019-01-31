// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkImageCreateFlags {
    pub sparse_binding: bool,
    pub sparse_residency: bool,
    pub sparse_aliased: bool,
    pub mutable_format: bool,
    pub cube_compatible: bool,
    pub alias: bool,
    pub split_instance_bind_regions: bool,
    pub _2d_array_compatible: bool,
    pub block_texel_view_compatible: bool,
    pub extended_usage: bool,
    pub protected: bool,
    pub disjoint: bool,
    pub corner_sampled_nv: bool,
    pub sample_locations_compatible_depth_ext: bool,
    pub subsampled_ext: bool,
}

pub type RawVkImageCreateFlags = u32;

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
        if src.corner_sampled_nv { *dst |= 0x00002000; }
        if src.sample_locations_compatible_depth_ext { *dst |= 0x00001000; }
        if src.subsampled_ext { *dst |= 0x00004000; }
    }
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
            corner_sampled_nv: (src & 0x00002000) != 0,
            sample_locations_compatible_depth_ext: (src & 0x00001000) != 0,
            subsampled_ext: (src & 0x00004000) != 0,
        }
    }
}

impl Default for VkImageCreateFlags {
    fn default() -> VkImageCreateFlags {
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
            corner_sampled_nv: false,
            sample_locations_compatible_depth_ext: false,
            subsampled_ext: false,
        }
    }
}

impl VkImageCreateFlags {
    
    pub fn none() -> VkImageCreateFlags {
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
            corner_sampled_nv: false,
            sample_locations_compatible_depth_ext: false,
            subsampled_ext: false,
        }
    }
    
    pub fn all() -> VkImageCreateFlags {
        VkImageCreateFlags {
            sparse_binding: true,
            sparse_residency: true,
            sparse_aliased: true,
            mutable_format: true,
            cube_compatible: true,
            alias: true,
            split_instance_bind_regions: true,
            _2d_array_compatible: true,
            block_texel_view_compatible: true,
            extended_usage: true,
            protected: true,
            disjoint: true,
            corner_sampled_nv: true,
            sample_locations_compatible_depth_ext: true,
            subsampled_ext: true,
        }
    }
}

#[macro_export]
macro_rules! VkImageCreateFlags {
    ( $( $x:ident ),* ) => {
        VkImageCreateFlags {
            $($x: true,)*
            ..VkImageCreateFlags::none()
        }
    }
}

impl VkImageCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.sparse_binding { 0x00000001 } else { 0 }
        + if self.sparse_residency { 0x00000002 } else { 0 }
        + if self.sparse_aliased { 0x00000004 } else { 0 }
        + if self.mutable_format { 0x00000008 } else { 0 }
        + if self.cube_compatible { 0x00000010 } else { 0 }
        + if self.alias { 0x00000400 } else { 0 }
        + if self.split_instance_bind_regions { 0x00000040 } else { 0 }
        + if self._2d_array_compatible { 0x00000020 } else { 0 }
        + if self.block_texel_view_compatible { 0x00000080 } else { 0 }
        + if self.extended_usage { 0x00000100 } else { 0 }
        + if self.protected { 0x00000800 } else { 0 }
        + if self.disjoint { 0x00000200 } else { 0 }
        + if self.corner_sampled_nv { 0x00002000 } else { 0 }
        + if self.sample_locations_compatible_depth_ext { 0x00001000 } else { 0 }
        + if self.subsampled_ext { 0x00004000 } else { 0 }
    }
}