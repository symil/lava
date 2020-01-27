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

/// Wrapper for [VkPhysicalDeviceVulkan11Features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan11Features.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceVulkan11Features {
    pub storage_buffer_16_bit_access: bool,
    pub uniform_and_storage_buffer_16_bit_access: bool,
    pub storage_push_constant_16: bool,
    pub storage_input_output_16: bool,
    pub multiview: bool,
    pub multiview_geometry_shader: bool,
    pub multiview_tessellation_shader: bool,
    pub variable_pointers_storage_buffer: bool,
    pub variable_pointers: bool,
    pub protected_memory: bool,
    pub sampler_ycbcr_conversion: bool,
    pub shader_draw_parameters: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceVulkan11Features {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub storage_buffer_16_bit_access: u32,
    pub uniform_and_storage_buffer_16_bit_access: u32,
    pub storage_push_constant_16: u32,
    pub storage_input_output_16: u32,
    pub multiview: u32,
    pub multiview_geometry_shader: u32,
    pub multiview_tessellation_shader: u32,
    pub variable_pointers_storage_buffer: u32,
    pub variable_pointers: u32,
    pub protected_memory: u32,
    pub sampler_ycbcr_conversion: u32,
    pub shader_draw_parameters: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceVulkan11Features> for VkPhysicalDeviceVulkan11Features {
    fn vk_to_raw(src: &VkPhysicalDeviceVulkan11Features, dst: &mut RawVkPhysicalDeviceVulkan11Features) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceVulkan11Features);
        dst.next = ptr::null_mut();
        dst.storage_buffer_16_bit_access = vk_to_raw_value(&src.storage_buffer_16_bit_access);
        dst.uniform_and_storage_buffer_16_bit_access = vk_to_raw_value(&src.uniform_and_storage_buffer_16_bit_access);
        dst.storage_push_constant_16 = vk_to_raw_value(&src.storage_push_constant_16);
        dst.storage_input_output_16 = vk_to_raw_value(&src.storage_input_output_16);
        dst.multiview = vk_to_raw_value(&src.multiview);
        dst.multiview_geometry_shader = vk_to_raw_value(&src.multiview_geometry_shader);
        dst.multiview_tessellation_shader = vk_to_raw_value(&src.multiview_tessellation_shader);
        dst.variable_pointers_storage_buffer = vk_to_raw_value(&src.variable_pointers_storage_buffer);
        dst.variable_pointers = vk_to_raw_value(&src.variable_pointers);
        dst.protected_memory = vk_to_raw_value(&src.protected_memory);
        dst.sampler_ycbcr_conversion = vk_to_raw_value(&src.sampler_ycbcr_conversion);
        dst.shader_draw_parameters = vk_to_raw_value(&src.shader_draw_parameters);
    }
}

impl VkRawType<VkPhysicalDeviceVulkan11Features> for RawVkPhysicalDeviceVulkan11Features {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceVulkan11Features) -> VkPhysicalDeviceVulkan11Features {
        VkPhysicalDeviceVulkan11Features {
            storage_buffer_16_bit_access: u32::vk_to_wrapped(&src.storage_buffer_16_bit_access),
            uniform_and_storage_buffer_16_bit_access: u32::vk_to_wrapped(&src.uniform_and_storage_buffer_16_bit_access),
            storage_push_constant_16: u32::vk_to_wrapped(&src.storage_push_constant_16),
            storage_input_output_16: u32::vk_to_wrapped(&src.storage_input_output_16),
            multiview: u32::vk_to_wrapped(&src.multiview),
            multiview_geometry_shader: u32::vk_to_wrapped(&src.multiview_geometry_shader),
            multiview_tessellation_shader: u32::vk_to_wrapped(&src.multiview_tessellation_shader),
            variable_pointers_storage_buffer: u32::vk_to_wrapped(&src.variable_pointers_storage_buffer),
            variable_pointers: u32::vk_to_wrapped(&src.variable_pointers),
            protected_memory: u32::vk_to_wrapped(&src.protected_memory),
            sampler_ycbcr_conversion: u32::vk_to_wrapped(&src.sampler_ycbcr_conversion),
            shader_draw_parameters: u32::vk_to_wrapped(&src.shader_draw_parameters),
        }
    }
}

impl Default for VkPhysicalDeviceVulkan11Features {
    fn default() -> VkPhysicalDeviceVulkan11Features {
        VkPhysicalDeviceVulkan11Features {
            storage_buffer_16_bit_access: false,
            uniform_and_storage_buffer_16_bit_access: false,
            storage_push_constant_16: false,
            storage_input_output_16: false,
            multiview: false,
            multiview_geometry_shader: false,
            multiview_tessellation_shader: false,
            variable_pointers_storage_buffer: false,
            variable_pointers: false,
            protected_memory: false,
            sampler_ycbcr_conversion: false,
            shader_draw_parameters: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceVulkan11Features {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceVulkan11Features {
    fn vk_free(&self) {
        
    }
}