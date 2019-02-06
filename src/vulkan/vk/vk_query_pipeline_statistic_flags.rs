// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkQueryPipelineStatisticFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPipelineStatisticFlagBits.html).
///
/// Use the macro `VkQueryPipelineStatisticFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkQueryPipelineStatisticFlags!(input_assembly_vertices, input_assembly_primitives)
/// ```
/// ```
/// VkQueryPipelineStatisticFlags {
///     input_assembly_vertices: true,
///     input_assembly_primitives: true,
///     ..VkQueryPipelineStatisticFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkQueryPipelineStatisticFlags {
    pub input_assembly_vertices: bool,
    pub input_assembly_primitives: bool,
    pub vertex_shader_invocations: bool,
    pub geometry_shader_invocations: bool,
    pub geometry_shader_primitives: bool,
    pub clipping_invocations: bool,
    pub clipping_primitives: bool,
    pub fragment_shader_invocations: bool,
    pub tessellation_control_shader_patches: bool,
    pub tessellation_evaluation_shader_invocations: bool,
    pub compute_shader_invocations: bool,
}

#[doc(hidden)]
pub type RawVkQueryPipelineStatisticFlags = u32;

impl VkWrappedType<RawVkQueryPipelineStatisticFlags> for VkQueryPipelineStatisticFlags {
    fn vk_to_raw(src: &VkQueryPipelineStatisticFlags, dst: &mut RawVkQueryPipelineStatisticFlags) {
        *dst = 0;
        if src.input_assembly_vertices { *dst |= 0x00000001; }
        if src.input_assembly_primitives { *dst |= 0x00000002; }
        if src.vertex_shader_invocations { *dst |= 0x00000004; }
        if src.geometry_shader_invocations { *dst |= 0x00000008; }
        if src.geometry_shader_primitives { *dst |= 0x00000010; }
        if src.clipping_invocations { *dst |= 0x00000020; }
        if src.clipping_primitives { *dst |= 0x00000040; }
        if src.fragment_shader_invocations { *dst |= 0x00000080; }
        if src.tessellation_control_shader_patches { *dst |= 0x00000100; }
        if src.tessellation_evaluation_shader_invocations { *dst |= 0x00000200; }
        if src.compute_shader_invocations { *dst |= 0x00000400; }
    }
}

impl VkRawType<VkQueryPipelineStatisticFlags> for RawVkQueryPipelineStatisticFlags {
    fn vk_to_wrapped(src: &RawVkQueryPipelineStatisticFlags) -> VkQueryPipelineStatisticFlags {
        VkQueryPipelineStatisticFlags {
            input_assembly_vertices: (src & 0x00000001) != 0,
            input_assembly_primitives: (src & 0x00000002) != 0,
            vertex_shader_invocations: (src & 0x00000004) != 0,
            geometry_shader_invocations: (src & 0x00000008) != 0,
            geometry_shader_primitives: (src & 0x00000010) != 0,
            clipping_invocations: (src & 0x00000020) != 0,
            clipping_primitives: (src & 0x00000040) != 0,
            fragment_shader_invocations: (src & 0x00000080) != 0,
            tessellation_control_shader_patches: (src & 0x00000100) != 0,
            tessellation_evaluation_shader_invocations: (src & 0x00000200) != 0,
            compute_shader_invocations: (src & 0x00000400) != 0,
        }
    }
}

impl Default for VkQueryPipelineStatisticFlags {
    fn default() -> VkQueryPipelineStatisticFlags {
        VkQueryPipelineStatisticFlags {
            input_assembly_vertices: false,
            input_assembly_primitives: false,
            vertex_shader_invocations: false,
            geometry_shader_invocations: false,
            geometry_shader_primitives: false,
            clipping_invocations: false,
            clipping_primitives: false,
            fragment_shader_invocations: false,
            tessellation_control_shader_patches: false,
            tessellation_evaluation_shader_invocations: false,
            compute_shader_invocations: false,
        }
    }
}

impl VkQueryPipelineStatisticFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkQueryPipelineStatisticFlags {
            input_assembly_vertices: false,
            input_assembly_primitives: false,
            vertex_shader_invocations: false,
            geometry_shader_invocations: false,
            geometry_shader_primitives: false,
            clipping_invocations: false,
            clipping_primitives: false,
            fragment_shader_invocations: false,
            tessellation_control_shader_patches: false,
            tessellation_evaluation_shader_invocations: false,
            compute_shader_invocations: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkQueryPipelineStatisticFlags {
            input_assembly_vertices: true,
            input_assembly_primitives: true,
            vertex_shader_invocations: true,
            geometry_shader_invocations: true,
            geometry_shader_primitives: true,
            clipping_invocations: true,
            clipping_primitives: true,
            fragment_shader_invocations: true,
            tessellation_control_shader_patches: true,
            tessellation_evaluation_shader_invocations: true,
            compute_shader_invocations: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.input_assembly_vertices { 0x00000001 } else { 0 }
        + if self.input_assembly_primitives { 0x00000002 } else { 0 }
        + if self.vertex_shader_invocations { 0x00000004 } else { 0 }
        + if self.geometry_shader_invocations { 0x00000008 } else { 0 }
        + if self.geometry_shader_primitives { 0x00000010 } else { 0 }
        + if self.clipping_invocations { 0x00000020 } else { 0 }
        + if self.clipping_primitives { 0x00000040 } else { 0 }
        + if self.fragment_shader_invocations { 0x00000080 } else { 0 }
        + if self.tessellation_control_shader_patches { 0x00000100 } else { 0 }
        + if self.tessellation_evaluation_shader_invocations { 0x00000200 } else { 0 }
        + if self.compute_shader_invocations { 0x00000400 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkQueryPipelineStatisticFlags {
            input_assembly_vertices: value & 0x00000001 > 0,
            input_assembly_primitives: value & 0x00000002 > 0,
            vertex_shader_invocations: value & 0x00000004 > 0,
            geometry_shader_invocations: value & 0x00000008 > 0,
            geometry_shader_primitives: value & 0x00000010 > 0,
            clipping_invocations: value & 0x00000020 > 0,
            clipping_primitives: value & 0x00000040 > 0,
            fragment_shader_invocations: value & 0x00000080 > 0,
            tessellation_control_shader_patches: value & 0x00000100 > 0,
            tessellation_evaluation_shader_invocations: value & 0x00000200 > 0,
            compute_shader_invocations: value & 0x00000400 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkQueryPipelineStatisticFlags {
    ( $( $x:ident ),* ) => {
        VkQueryPipelineStatisticFlags {
            $($x: true,)*
            ..VkQueryPipelineStatisticFlags::none()
        }
    }
}