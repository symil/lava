// Generated by `scripts/generate.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};

/// Wrapper for [VkPhysicalDeviceConservativeRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceConservativeRasterizationProperties {
    pub primitive_overestimation_size: f32,
    pub max_extra_primitive_overestimation_size: f32,
    pub extra_primitive_overestimation_size_granularity: f32,
    pub primitive_underestimation: bool,
    pub conservative_point_and_line_rasterization: bool,
    pub degenerate_triangles_rasterized: bool,
    pub degenerate_lines_rasterized: bool,
    pub fully_covered_fragment_shader_input_variable: bool,
    pub conservative_rasterization_post_depth_coverage: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceConservativeRasterizationProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub primitive_overestimation_size: f32,
    pub max_extra_primitive_overestimation_size: f32,
    pub extra_primitive_overestimation_size_granularity: f32,
    pub primitive_underestimation: u32,
    pub conservative_point_and_line_rasterization: u32,
    pub degenerate_triangles_rasterized: u32,
    pub degenerate_lines_rasterized: u32,
    pub fully_covered_fragment_shader_input_variable: u32,
    pub conservative_rasterization_post_depth_coverage: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceConservativeRasterizationProperties> for VkPhysicalDeviceConservativeRasterizationProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceConservativeRasterizationProperties, dst: &mut RawVkPhysicalDeviceConservativeRasterizationProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceConservativeRasterizationPropertiesExt);
        dst.next = ptr::null_mut();
        dst.primitive_overestimation_size = src.primitive_overestimation_size;
        dst.max_extra_primitive_overestimation_size = src.max_extra_primitive_overestimation_size;
        dst.extra_primitive_overestimation_size_granularity = src.extra_primitive_overestimation_size_granularity;
        dst.primitive_underestimation = vk_to_raw_value(&src.primitive_underestimation);
        dst.conservative_point_and_line_rasterization = vk_to_raw_value(&src.conservative_point_and_line_rasterization);
        dst.degenerate_triangles_rasterized = vk_to_raw_value(&src.degenerate_triangles_rasterized);
        dst.degenerate_lines_rasterized = vk_to_raw_value(&src.degenerate_lines_rasterized);
        dst.fully_covered_fragment_shader_input_variable = vk_to_raw_value(&src.fully_covered_fragment_shader_input_variable);
        dst.conservative_rasterization_post_depth_coverage = vk_to_raw_value(&src.conservative_rasterization_post_depth_coverage);
    }
}

impl VkRawType<VkPhysicalDeviceConservativeRasterizationProperties> for RawVkPhysicalDeviceConservativeRasterizationProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceConservativeRasterizationProperties) -> VkPhysicalDeviceConservativeRasterizationProperties {
        VkPhysicalDeviceConservativeRasterizationProperties {
            primitive_overestimation_size: src.primitive_overestimation_size,
            max_extra_primitive_overestimation_size: src.max_extra_primitive_overestimation_size,
            extra_primitive_overestimation_size_granularity: src.extra_primitive_overestimation_size_granularity,
            primitive_underestimation: u32::vk_to_wrapped(&src.primitive_underestimation),
            conservative_point_and_line_rasterization: u32::vk_to_wrapped(&src.conservative_point_and_line_rasterization),
            degenerate_triangles_rasterized: u32::vk_to_wrapped(&src.degenerate_triangles_rasterized),
            degenerate_lines_rasterized: u32::vk_to_wrapped(&src.degenerate_lines_rasterized),
            fully_covered_fragment_shader_input_variable: u32::vk_to_wrapped(&src.fully_covered_fragment_shader_input_variable),
            conservative_rasterization_post_depth_coverage: u32::vk_to_wrapped(&src.conservative_rasterization_post_depth_coverage),
        }
    }
}

impl Default for VkPhysicalDeviceConservativeRasterizationProperties {
    fn default() -> VkPhysicalDeviceConservativeRasterizationProperties {
        VkPhysicalDeviceConservativeRasterizationProperties {
            primitive_overestimation_size: 0.0,
            max_extra_primitive_overestimation_size: 0.0,
            extra_primitive_overestimation_size_granularity: 0.0,
            primitive_underestimation: false,
            conservative_point_and_line_rasterization: false,
            degenerate_triangles_rasterized: false,
            degenerate_lines_rasterized: false,
            fully_covered_fragment_shader_input_variable: false,
            conservative_rasterization_post_depth_coverage: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceConservativeRasterizationProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceConservativeRasterizationProperties {
    fn vk_free(&self) {
        
    }
}