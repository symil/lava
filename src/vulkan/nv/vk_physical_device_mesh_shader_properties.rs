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

/// Wrapper for [VkPhysicalDeviceMeshShaderPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMeshShaderProperties {
    pub max_draw_mesh_tasks_count: usize,
    pub max_task_work_group_invocations: usize,
    pub max_task_work_group_size: [usize; 3],
    pub max_task_total_memory_size: usize,
    pub max_task_output_count: usize,
    pub max_mesh_work_group_invocations: usize,
    pub max_mesh_work_group_size: [usize; 3],
    pub max_mesh_total_memory_size: usize,
    pub max_mesh_output_vertices: usize,
    pub max_mesh_output_primitives: usize,
    pub max_mesh_multiview_view_count: usize,
    pub mesh_output_per_vertex_granularity: usize,
    pub mesh_output_per_primitive_granularity: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMeshShaderProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub max_draw_mesh_tasks_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_total_memory_size: u32,
    pub max_task_output_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_total_memory_size: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceMeshShaderProperties> for VkPhysicalDeviceMeshShaderProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceMeshShaderProperties, dst: &mut RawVkPhysicalDeviceMeshShaderProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMeshShaderPropertiesNv);
        dst.next = ptr::null_mut();
        dst.max_draw_mesh_tasks_count = vk_to_raw_value(&src.max_draw_mesh_tasks_count);
        dst.max_task_work_group_invocations = vk_to_raw_value(&src.max_task_work_group_invocations);
        dst.max_task_work_group_size = unsafe { let mut dst_array : [u32; 3] = mem::MaybeUninit::uninit().assume_init(); vk_to_raw_array(&src.max_task_work_group_size, &mut dst_array); dst_array };
        dst.max_task_total_memory_size = vk_to_raw_value(&src.max_task_total_memory_size);
        dst.max_task_output_count = vk_to_raw_value(&src.max_task_output_count);
        dst.max_mesh_work_group_invocations = vk_to_raw_value(&src.max_mesh_work_group_invocations);
        dst.max_mesh_work_group_size = unsafe { let mut dst_array : [u32; 3] = mem::MaybeUninit::uninit().assume_init(); vk_to_raw_array(&src.max_mesh_work_group_size, &mut dst_array); dst_array };
        dst.max_mesh_total_memory_size = vk_to_raw_value(&src.max_mesh_total_memory_size);
        dst.max_mesh_output_vertices = vk_to_raw_value(&src.max_mesh_output_vertices);
        dst.max_mesh_output_primitives = vk_to_raw_value(&src.max_mesh_output_primitives);
        dst.max_mesh_multiview_view_count = vk_to_raw_value(&src.max_mesh_multiview_view_count);
        dst.mesh_output_per_vertex_granularity = vk_to_raw_value(&src.mesh_output_per_vertex_granularity);
        dst.mesh_output_per_primitive_granularity = vk_to_raw_value(&src.mesh_output_per_primitive_granularity);
    }
}

impl VkRawType<VkPhysicalDeviceMeshShaderProperties> for RawVkPhysicalDeviceMeshShaderProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMeshShaderProperties) -> VkPhysicalDeviceMeshShaderProperties {
        VkPhysicalDeviceMeshShaderProperties {
            max_draw_mesh_tasks_count: u32::vk_to_wrapped(&src.max_draw_mesh_tasks_count),
            max_task_work_group_invocations: u32::vk_to_wrapped(&src.max_task_work_group_invocations),
            max_task_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::MaybeUninit::uninit().assume_init(); vk_to_wrapped_array(&src.max_task_work_group_size, &mut dst_array); dst_array },
            max_task_total_memory_size: u32::vk_to_wrapped(&src.max_task_total_memory_size),
            max_task_output_count: u32::vk_to_wrapped(&src.max_task_output_count),
            max_mesh_work_group_invocations: u32::vk_to_wrapped(&src.max_mesh_work_group_invocations),
            max_mesh_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::MaybeUninit::uninit().assume_init(); vk_to_wrapped_array(&src.max_mesh_work_group_size, &mut dst_array); dst_array },
            max_mesh_total_memory_size: u32::vk_to_wrapped(&src.max_mesh_total_memory_size),
            max_mesh_output_vertices: u32::vk_to_wrapped(&src.max_mesh_output_vertices),
            max_mesh_output_primitives: u32::vk_to_wrapped(&src.max_mesh_output_primitives),
            max_mesh_multiview_view_count: u32::vk_to_wrapped(&src.max_mesh_multiview_view_count),
            mesh_output_per_vertex_granularity: u32::vk_to_wrapped(&src.mesh_output_per_vertex_granularity),
            mesh_output_per_primitive_granularity: u32::vk_to_wrapped(&src.mesh_output_per_primitive_granularity),
        }
    }
}

impl Default for VkPhysicalDeviceMeshShaderProperties {
    fn default() -> VkPhysicalDeviceMeshShaderProperties {
        VkPhysicalDeviceMeshShaderProperties {
            max_draw_mesh_tasks_count: 0,
            max_task_work_group_invocations: 0,
            max_task_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::MaybeUninit::uninit().assume_init(); fill_vk_array(&mut dst_array); dst_array },
            max_task_total_memory_size: 0,
            max_task_output_count: 0,
            max_mesh_work_group_invocations: 0,
            max_mesh_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::MaybeUninit::uninit().assume_init(); fill_vk_array(&mut dst_array); dst_array },
            max_mesh_total_memory_size: 0,
            max_mesh_output_vertices: 0,
            max_mesh_output_primitives: 0,
            max_mesh_multiview_view_count: 0,
            mesh_output_per_vertex_granularity: 0,
            mesh_output_per_primitive_granularity: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceMeshShaderProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMeshShaderProperties {
    fn vk_free(&self) {
        
    }
}