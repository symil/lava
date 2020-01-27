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

/// Wrapper for [VkPhysicalDeviceDescriptorIndexingFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
    pub shader_input_attachment_array_dynamic_indexing: bool,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
    pub shader_uniform_buffer_array_non_uniform_indexing: bool,
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    pub shader_storage_buffer_array_non_uniform_indexing: bool,
    pub shader_storage_image_array_non_uniform_indexing: bool,
    pub shader_input_attachment_array_non_uniform_indexing: bool,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    pub descriptor_binding_uniform_buffer_update_after_bind: bool,
    pub descriptor_binding_sampled_image_update_after_bind: bool,
    pub descriptor_binding_storage_image_update_after_bind: bool,
    pub descriptor_binding_storage_buffer_update_after_bind: bool,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    pub descriptor_binding_update_unused_while_pending: bool,
    pub descriptor_binding_partially_bound: bool,
    pub descriptor_binding_variable_descriptor_count: bool,
    pub runtime_descriptor_array: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub shader_input_attachment_array_dynamic_indexing: u32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: u32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing: u32,
    pub shader_sampled_image_array_non_uniform_indexing: u32,
    pub shader_storage_buffer_array_non_uniform_indexing: u32,
    pub shader_storage_image_array_non_uniform_indexing: u32,
    pub shader_input_attachment_array_non_uniform_indexing: u32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: u32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: u32,
    pub descriptor_binding_uniform_buffer_update_after_bind: u32,
    pub descriptor_binding_sampled_image_update_after_bind: u32,
    pub descriptor_binding_storage_image_update_after_bind: u32,
    pub descriptor_binding_storage_buffer_update_after_bind: u32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: u32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: u32,
    pub descriptor_binding_update_unused_while_pending: u32,
    pub descriptor_binding_partially_bound: u32,
    pub descriptor_binding_variable_descriptor_count: u32,
    pub runtime_descriptor_array: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDescriptorIndexingFeatures> for VkPhysicalDeviceDescriptorIndexingFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceDescriptorIndexingFeatures, dst: &mut RawVkPhysicalDeviceDescriptorIndexingFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDescriptorIndexingFeatures);
        dst.next = ptr::null_mut();
        dst.shader_input_attachment_array_dynamic_indexing = vk_to_raw_value(&src.shader_input_attachment_array_dynamic_indexing);
        dst.shader_uniform_texel_buffer_array_dynamic_indexing = vk_to_raw_value(&src.shader_uniform_texel_buffer_array_dynamic_indexing);
        dst.shader_storage_texel_buffer_array_dynamic_indexing = vk_to_raw_value(&src.shader_storage_texel_buffer_array_dynamic_indexing);
        dst.shader_uniform_buffer_array_non_uniform_indexing = vk_to_raw_value(&src.shader_uniform_buffer_array_non_uniform_indexing);
        dst.shader_sampled_image_array_non_uniform_indexing = vk_to_raw_value(&src.shader_sampled_image_array_non_uniform_indexing);
        dst.shader_storage_buffer_array_non_uniform_indexing = vk_to_raw_value(&src.shader_storage_buffer_array_non_uniform_indexing);
        dst.shader_storage_image_array_non_uniform_indexing = vk_to_raw_value(&src.shader_storage_image_array_non_uniform_indexing);
        dst.shader_input_attachment_array_non_uniform_indexing = vk_to_raw_value(&src.shader_input_attachment_array_non_uniform_indexing);
        dst.shader_uniform_texel_buffer_array_non_uniform_indexing = vk_to_raw_value(&src.shader_uniform_texel_buffer_array_non_uniform_indexing);
        dst.shader_storage_texel_buffer_array_non_uniform_indexing = vk_to_raw_value(&src.shader_storage_texel_buffer_array_non_uniform_indexing);
        dst.descriptor_binding_uniform_buffer_update_after_bind = vk_to_raw_value(&src.descriptor_binding_uniform_buffer_update_after_bind);
        dst.descriptor_binding_sampled_image_update_after_bind = vk_to_raw_value(&src.descriptor_binding_sampled_image_update_after_bind);
        dst.descriptor_binding_storage_image_update_after_bind = vk_to_raw_value(&src.descriptor_binding_storage_image_update_after_bind);
        dst.descriptor_binding_storage_buffer_update_after_bind = vk_to_raw_value(&src.descriptor_binding_storage_buffer_update_after_bind);
        dst.descriptor_binding_uniform_texel_buffer_update_after_bind = vk_to_raw_value(&src.descriptor_binding_uniform_texel_buffer_update_after_bind);
        dst.descriptor_binding_storage_texel_buffer_update_after_bind = vk_to_raw_value(&src.descriptor_binding_storage_texel_buffer_update_after_bind);
        dst.descriptor_binding_update_unused_while_pending = vk_to_raw_value(&src.descriptor_binding_update_unused_while_pending);
        dst.descriptor_binding_partially_bound = vk_to_raw_value(&src.descriptor_binding_partially_bound);
        dst.descriptor_binding_variable_descriptor_count = vk_to_raw_value(&src.descriptor_binding_variable_descriptor_count);
        dst.runtime_descriptor_array = vk_to_raw_value(&src.runtime_descriptor_array);
    }
}

impl VkRawType<VkPhysicalDeviceDescriptorIndexingFeatures> for RawVkPhysicalDeviceDescriptorIndexingFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDescriptorIndexingFeatures) -> VkPhysicalDeviceDescriptorIndexingFeatures {
        VkPhysicalDeviceDescriptorIndexingFeatures {
            shader_input_attachment_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_input_attachment_array_dynamic_indexing),
            shader_uniform_texel_buffer_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_uniform_texel_buffer_array_dynamic_indexing),
            shader_storage_texel_buffer_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_storage_texel_buffer_array_dynamic_indexing),
            shader_uniform_buffer_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_uniform_buffer_array_non_uniform_indexing),
            shader_sampled_image_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_sampled_image_array_non_uniform_indexing),
            shader_storage_buffer_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_storage_buffer_array_non_uniform_indexing),
            shader_storage_image_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_storage_image_array_non_uniform_indexing),
            shader_input_attachment_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_input_attachment_array_non_uniform_indexing),
            shader_uniform_texel_buffer_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_uniform_texel_buffer_array_non_uniform_indexing),
            shader_storage_texel_buffer_array_non_uniform_indexing: u32::vk_to_wrapped(&src.shader_storage_texel_buffer_array_non_uniform_indexing),
            descriptor_binding_uniform_buffer_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_uniform_buffer_update_after_bind),
            descriptor_binding_sampled_image_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_sampled_image_update_after_bind),
            descriptor_binding_storage_image_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_storage_image_update_after_bind),
            descriptor_binding_storage_buffer_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_storage_buffer_update_after_bind),
            descriptor_binding_uniform_texel_buffer_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_uniform_texel_buffer_update_after_bind),
            descriptor_binding_storage_texel_buffer_update_after_bind: u32::vk_to_wrapped(&src.descriptor_binding_storage_texel_buffer_update_after_bind),
            descriptor_binding_update_unused_while_pending: u32::vk_to_wrapped(&src.descriptor_binding_update_unused_while_pending),
            descriptor_binding_partially_bound: u32::vk_to_wrapped(&src.descriptor_binding_partially_bound),
            descriptor_binding_variable_descriptor_count: u32::vk_to_wrapped(&src.descriptor_binding_variable_descriptor_count),
            runtime_descriptor_array: u32::vk_to_wrapped(&src.runtime_descriptor_array),
        }
    }
}

impl Default for VkPhysicalDeviceDescriptorIndexingFeatures {
    fn default() -> VkPhysicalDeviceDescriptorIndexingFeatures {
        VkPhysicalDeviceDescriptorIndexingFeatures {
            shader_input_attachment_array_dynamic_indexing: false,
            shader_uniform_texel_buffer_array_dynamic_indexing: false,
            shader_storage_texel_buffer_array_dynamic_indexing: false,
            shader_uniform_buffer_array_non_uniform_indexing: false,
            shader_sampled_image_array_non_uniform_indexing: false,
            shader_storage_buffer_array_non_uniform_indexing: false,
            shader_storage_image_array_non_uniform_indexing: false,
            shader_input_attachment_array_non_uniform_indexing: false,
            shader_uniform_texel_buffer_array_non_uniform_indexing: false,
            shader_storage_texel_buffer_array_non_uniform_indexing: false,
            descriptor_binding_uniform_buffer_update_after_bind: false,
            descriptor_binding_sampled_image_update_after_bind: false,
            descriptor_binding_storage_image_update_after_bind: false,
            descriptor_binding_storage_buffer_update_after_bind: false,
            descriptor_binding_uniform_texel_buffer_update_after_bind: false,
            descriptor_binding_storage_texel_buffer_update_after_bind: false,
            descriptor_binding_update_unused_while_pending: false,
            descriptor_binding_partially_bound: false,
            descriptor_binding_variable_descriptor_count: false,
            runtime_descriptor_array: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDescriptorIndexingFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDescriptorIndexingFeatures {
    fn vk_free(&self) {
        
    }
}