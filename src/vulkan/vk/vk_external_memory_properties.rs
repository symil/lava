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
use vulkan::vk::{VkExternalMemoryFeatureFlags,RawVkExternalMemoryFeatureFlags};
use vulkan::vk::{VkExternalMemoryHandleTypeFlags,RawVkExternalMemoryHandleTypeFlags};

/// Wrapper for [VkExternalMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryProperties.html).
#[derive(Debug, Clone)]
pub struct VkExternalMemoryProperties {
    pub external_memory_features: VkExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: VkExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: VkExternalMemoryHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExternalMemoryProperties {
    pub external_memory_features: RawVkExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: RawVkExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: RawVkExternalMemoryHandleTypeFlags,
}

impl VkWrappedType<RawVkExternalMemoryProperties> for VkExternalMemoryProperties {
    fn vk_to_raw(src: &VkExternalMemoryProperties, dst: &mut RawVkExternalMemoryProperties) {
        dst.external_memory_features = vk_to_raw_value(&src.external_memory_features);
        dst.export_from_imported_handle_types = vk_to_raw_value(&src.export_from_imported_handle_types);
        dst.compatible_handle_types = vk_to_raw_value(&src.compatible_handle_types);
    }
}

impl VkRawType<VkExternalMemoryProperties> for RawVkExternalMemoryProperties {
    fn vk_to_wrapped(src: &RawVkExternalMemoryProperties) -> VkExternalMemoryProperties {
        VkExternalMemoryProperties {
            external_memory_features: RawVkExternalMemoryFeatureFlags::vk_to_wrapped(&src.external_memory_features),
            export_from_imported_handle_types: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.export_from_imported_handle_types),
            compatible_handle_types: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.compatible_handle_types),
        }
    }
}

impl Default for VkExternalMemoryProperties {
    fn default() -> VkExternalMemoryProperties {
        VkExternalMemoryProperties {
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}

impl VkSetup for VkExternalMemoryProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkExternalMemoryProperties {
    fn vk_free(&self) {
        
    }
}