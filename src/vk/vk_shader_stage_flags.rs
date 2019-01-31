// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkShaderStageFlags {
    pub vertex: bool,
    pub tessellation_control: bool,
    pub tessellation_evaluation: bool,
    pub geometry: bool,
    pub fragment: bool,
    pub compute: bool,
    pub all_graphics: bool,
    pub raygen_nv: bool,
    pub any_hit_nv: bool,
    pub closest_hit_nv: bool,
    pub miss_nv: bool,
    pub intersection_nv: bool,
    pub callable_nv: bool,
    pub task_nv: bool,
    pub mesh_nv: bool,
}

pub type RawVkShaderStageFlags = u32;

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
        if src.raygen_nv { *dst |= 0x00000100; }
        if src.any_hit_nv { *dst |= 0x00000200; }
        if src.closest_hit_nv { *dst |= 0x00000400; }
        if src.miss_nv { *dst |= 0x00000800; }
        if src.intersection_nv { *dst |= 0x00001000; }
        if src.callable_nv { *dst |= 0x00002000; }
        if src.task_nv { *dst |= 0x00000040; }
        if src.mesh_nv { *dst |= 0x00000080; }
    }
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
            raygen_nv: (src & 0x00000100) != 0,
            any_hit_nv: (src & 0x00000200) != 0,
            closest_hit_nv: (src & 0x00000400) != 0,
            miss_nv: (src & 0x00000800) != 0,
            intersection_nv: (src & 0x00001000) != 0,
            callable_nv: (src & 0x00002000) != 0,
            task_nv: (src & 0x00000040) != 0,
            mesh_nv: (src & 0x00000080) != 0,
        }
    }
}

impl Default for VkShaderStageFlags {
    fn default() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: false,
            tessellation_control: false,
            tessellation_evaluation: false,
            geometry: false,
            fragment: false,
            compute: false,
            all_graphics: false,
            raygen_nv: false,
            any_hit_nv: false,
            closest_hit_nv: false,
            miss_nv: false,
            intersection_nv: false,
            callable_nv: false,
            task_nv: false,
            mesh_nv: false,
        }
    }
}

impl VkShaderStageFlags {
    
    pub fn none() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: false,
            tessellation_control: false,
            tessellation_evaluation: false,
            geometry: false,
            fragment: false,
            compute: false,
            all_graphics: false,
            raygen_nv: false,
            any_hit_nv: false,
            closest_hit_nv: false,
            miss_nv: false,
            intersection_nv: false,
            callable_nv: false,
            task_nv: false,
            mesh_nv: false,
        }
    }
    
    pub fn all() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: true,
            tessellation_control: true,
            tessellation_evaluation: true,
            geometry: true,
            fragment: true,
            compute: true,
            all_graphics: true,
            raygen_nv: true,
            any_hit_nv: true,
            closest_hit_nv: true,
            miss_nv: true,
            intersection_nv: true,
            callable_nv: true,
            task_nv: true,
            mesh_nv: true,
        }
    }
}

#[macro_export]
macro_rules! VkShaderStageFlags {
    ( $( $x:ident ),* ) => {
        VkShaderStageFlags {
            $($x: true,)*
            ..VkShaderStageFlags::none()
        }
    }
}

impl VkShaderStageFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.vertex { 0x00000001 } else { 0 }
        + if self.tessellation_control { 0x00000002 } else { 0 }
        + if self.tessellation_evaluation { 0x00000004 } else { 0 }
        + if self.geometry { 0x00000008 } else { 0 }
        + if self.fragment { 0x00000010 } else { 0 }
        + if self.compute { 0x00000020 } else { 0 }
        + if self.all_graphics { 0x0000001F } else { 0 }
        + if self.raygen_nv { 0x00000100 } else { 0 }
        + if self.any_hit_nv { 0x00000200 } else { 0 }
        + if self.closest_hit_nv { 0x00000400 } else { 0 }
        + if self.miss_nv { 0x00000800 } else { 0 }
        + if self.intersection_nv { 0x00001000 } else { 0 }
        + if self.callable_nv { 0x00002000 } else { 0 }
        + if self.task_nv { 0x00000040 } else { 0 }
        + if self.mesh_nv { 0x00000080 } else { 0 }
    }
}