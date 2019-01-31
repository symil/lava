// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineStageFlags {
    pub top_of_pipe: bool,
    pub draw_indirect: bool,
    pub vertex_input: bool,
    pub vertex_shader: bool,
    pub tessellation_control_shader: bool,
    pub tessellation_evaluation_shader: bool,
    pub geometry_shader: bool,
    pub fragment_shader: bool,
    pub early_fragment_tests: bool,
    pub late_fragment_tests: bool,
    pub color_attachment_output: bool,
    pub compute_shader: bool,
    pub transfer: bool,
    pub bottom_of_pipe: bool,
    pub host: bool,
    pub all_graphics: bool,
    pub all_commands: bool,
    pub transform_feedback_ext: bool,
    pub conditional_rendering_ext: bool,
    pub command_process_nvx: bool,
    pub shading_rate_image_nv: bool,
    pub ray_tracing_shader_nv: bool,
    pub acceleration_structure_build_nv: bool,
    pub task_shader_nv: bool,
    pub mesh_shader_nv: bool,
    pub fragment_density_process_ext: bool,
}

pub type RawVkPipelineStageFlags = u32;

impl VkWrappedType<RawVkPipelineStageFlags> for VkPipelineStageFlags {
    fn vk_to_raw(src: &VkPipelineStageFlags, dst: &mut RawVkPipelineStageFlags) {
        *dst = 0;
        if src.top_of_pipe { *dst |= 0x00000001; }
        if src.draw_indirect { *dst |= 0x00000002; }
        if src.vertex_input { *dst |= 0x00000004; }
        if src.vertex_shader { *dst |= 0x00000008; }
        if src.tessellation_control_shader { *dst |= 0x00000010; }
        if src.tessellation_evaluation_shader { *dst |= 0x00000020; }
        if src.geometry_shader { *dst |= 0x00000040; }
        if src.fragment_shader { *dst |= 0x00000080; }
        if src.early_fragment_tests { *dst |= 0x00000100; }
        if src.late_fragment_tests { *dst |= 0x00000200; }
        if src.color_attachment_output { *dst |= 0x00000400; }
        if src.compute_shader { *dst |= 0x00000800; }
        if src.transfer { *dst |= 0x00001000; }
        if src.bottom_of_pipe { *dst |= 0x00002000; }
        if src.host { *dst |= 0x00004000; }
        if src.all_graphics { *dst |= 0x00008000; }
        if src.all_commands { *dst |= 0x00010000; }
        if src.transform_feedback_ext { *dst |= 0x01000000; }
        if src.conditional_rendering_ext { *dst |= 0x00040000; }
        if src.command_process_nvx { *dst |= 0x00020000; }
        if src.shading_rate_image_nv { *dst |= 0x00400000; }
        if src.ray_tracing_shader_nv { *dst |= 0x00200000; }
        if src.acceleration_structure_build_nv { *dst |= 0x02000000; }
        if src.task_shader_nv { *dst |= 0x00080000; }
        if src.mesh_shader_nv { *dst |= 0x00100000; }
        if src.fragment_density_process_ext { *dst |= 0x00800000; }
    }
}

impl VkRawType<VkPipelineStageFlags> for RawVkPipelineStageFlags {
    fn vk_to_wrapped(src: &RawVkPipelineStageFlags) -> VkPipelineStageFlags {
        VkPipelineStageFlags {
            top_of_pipe: (src & 0x00000001) != 0,
            draw_indirect: (src & 0x00000002) != 0,
            vertex_input: (src & 0x00000004) != 0,
            vertex_shader: (src & 0x00000008) != 0,
            tessellation_control_shader: (src & 0x00000010) != 0,
            tessellation_evaluation_shader: (src & 0x00000020) != 0,
            geometry_shader: (src & 0x00000040) != 0,
            fragment_shader: (src & 0x00000080) != 0,
            early_fragment_tests: (src & 0x00000100) != 0,
            late_fragment_tests: (src & 0x00000200) != 0,
            color_attachment_output: (src & 0x00000400) != 0,
            compute_shader: (src & 0x00000800) != 0,
            transfer: (src & 0x00001000) != 0,
            bottom_of_pipe: (src & 0x00002000) != 0,
            host: (src & 0x00004000) != 0,
            all_graphics: (src & 0x00008000) != 0,
            all_commands: (src & 0x00010000) != 0,
            transform_feedback_ext: (src & 0x01000000) != 0,
            conditional_rendering_ext: (src & 0x00040000) != 0,
            command_process_nvx: (src & 0x00020000) != 0,
            shading_rate_image_nv: (src & 0x00400000) != 0,
            ray_tracing_shader_nv: (src & 0x00200000) != 0,
            acceleration_structure_build_nv: (src & 0x02000000) != 0,
            task_shader_nv: (src & 0x00080000) != 0,
            mesh_shader_nv: (src & 0x00100000) != 0,
            fragment_density_process_ext: (src & 0x00800000) != 0,
        }
    }
}

impl Default for VkPipelineStageFlags {
    fn default() -> VkPipelineStageFlags {
        VkPipelineStageFlags {
            top_of_pipe: false,
            draw_indirect: false,
            vertex_input: false,
            vertex_shader: false,
            tessellation_control_shader: false,
            tessellation_evaluation_shader: false,
            geometry_shader: false,
            fragment_shader: false,
            early_fragment_tests: false,
            late_fragment_tests: false,
            color_attachment_output: false,
            compute_shader: false,
            transfer: false,
            bottom_of_pipe: false,
            host: false,
            all_graphics: false,
            all_commands: false,
            transform_feedback_ext: false,
            conditional_rendering_ext: false,
            command_process_nvx: false,
            shading_rate_image_nv: false,
            ray_tracing_shader_nv: false,
            acceleration_structure_build_nv: false,
            task_shader_nv: false,
            mesh_shader_nv: false,
            fragment_density_process_ext: false,
        }
    }
}

impl VkPipelineStageFlags {
    
    pub fn none() -> VkPipelineStageFlags {
        VkPipelineStageFlags {
            top_of_pipe: false,
            draw_indirect: false,
            vertex_input: false,
            vertex_shader: false,
            tessellation_control_shader: false,
            tessellation_evaluation_shader: false,
            geometry_shader: false,
            fragment_shader: false,
            early_fragment_tests: false,
            late_fragment_tests: false,
            color_attachment_output: false,
            compute_shader: false,
            transfer: false,
            bottom_of_pipe: false,
            host: false,
            all_graphics: false,
            all_commands: false,
            transform_feedback_ext: false,
            conditional_rendering_ext: false,
            command_process_nvx: false,
            shading_rate_image_nv: false,
            ray_tracing_shader_nv: false,
            acceleration_structure_build_nv: false,
            task_shader_nv: false,
            mesh_shader_nv: false,
            fragment_density_process_ext: false,
        }
    }
    
    pub fn all() -> VkPipelineStageFlags {
        VkPipelineStageFlags {
            top_of_pipe: true,
            draw_indirect: true,
            vertex_input: true,
            vertex_shader: true,
            tessellation_control_shader: true,
            tessellation_evaluation_shader: true,
            geometry_shader: true,
            fragment_shader: true,
            early_fragment_tests: true,
            late_fragment_tests: true,
            color_attachment_output: true,
            compute_shader: true,
            transfer: true,
            bottom_of_pipe: true,
            host: true,
            all_graphics: true,
            all_commands: true,
            transform_feedback_ext: true,
            conditional_rendering_ext: true,
            command_process_nvx: true,
            shading_rate_image_nv: true,
            ray_tracing_shader_nv: true,
            acceleration_structure_build_nv: true,
            task_shader_nv: true,
            mesh_shader_nv: true,
            fragment_density_process_ext: true,
        }
    }
}

#[macro_export]
macro_rules! VkPipelineStageFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineStageFlags {
            $($x: true,)*
            ..VkPipelineStageFlags::none()
        }
    }
}

impl VkPipelineStageFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.top_of_pipe { 0x00000001 } else { 0 }
        + if self.draw_indirect { 0x00000002 } else { 0 }
        + if self.vertex_input { 0x00000004 } else { 0 }
        + if self.vertex_shader { 0x00000008 } else { 0 }
        + if self.tessellation_control_shader { 0x00000010 } else { 0 }
        + if self.tessellation_evaluation_shader { 0x00000020 } else { 0 }
        + if self.geometry_shader { 0x00000040 } else { 0 }
        + if self.fragment_shader { 0x00000080 } else { 0 }
        + if self.early_fragment_tests { 0x00000100 } else { 0 }
        + if self.late_fragment_tests { 0x00000200 } else { 0 }
        + if self.color_attachment_output { 0x00000400 } else { 0 }
        + if self.compute_shader { 0x00000800 } else { 0 }
        + if self.transfer { 0x00001000 } else { 0 }
        + if self.bottom_of_pipe { 0x00002000 } else { 0 }
        + if self.host { 0x00004000 } else { 0 }
        + if self.all_graphics { 0x00008000 } else { 0 }
        + if self.all_commands { 0x00010000 } else { 0 }
        + if self.transform_feedback_ext { 0x01000000 } else { 0 }
        + if self.conditional_rendering_ext { 0x00040000 } else { 0 }
        + if self.command_process_nvx { 0x00020000 } else { 0 }
        + if self.shading_rate_image_nv { 0x00400000 } else { 0 }
        + if self.ray_tracing_shader_nv { 0x00200000 } else { 0 }
        + if self.acceleration_structure_build_nv { 0x02000000 } else { 0 }
        + if self.task_shader_nv { 0x00080000 } else { 0 }
        + if self.mesh_shader_nv { 0x00100000 } else { 0 }
        + if self.fragment_density_process_ext { 0x00800000 } else { 0 }
    }
}