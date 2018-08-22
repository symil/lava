// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::vk_external_semaphore_handle_type_flags::*;
use vk::vk_external_semaphore_feature_flags::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExternalSemaphoreProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub export_from_imported_handle_types: RawVkExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: RawVkExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: RawVkExternalSemaphoreFeatureFlags,
}

#[derive(Debug, Clone)]
pub struct VkExternalSemaphoreProperties {
    pub export_from_imported_handle_types: VkExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: VkExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: VkExternalSemaphoreFeatureFlags,
}

impl VkRawType<VkExternalSemaphoreProperties> for RawVkExternalSemaphoreProperties {
    fn vk_to_wrapped(src: &RawVkExternalSemaphoreProperties) -> VkExternalSemaphoreProperties {
        VkExternalSemaphoreProperties {
            export_from_imported_handle_types: RawVkExternalSemaphoreHandleTypeFlags::vk_to_wrapped(&src.export_from_imported_handle_types),
            compatible_handle_types: RawVkExternalSemaphoreHandleTypeFlags::vk_to_wrapped(&src.compatible_handle_types),
            external_semaphore_features: RawVkExternalSemaphoreFeatureFlags::vk_to_wrapped(&src.external_semaphore_features),
        }
    }
}

impl VkWrappedType<RawVkExternalSemaphoreProperties> for VkExternalSemaphoreProperties {
    fn vk_to_raw(src: &VkExternalSemaphoreProperties, dst: &mut RawVkExternalSemaphoreProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExternalSemaphoreProperties);
        dst.next = ptr::null();
        dst.export_from_imported_handle_types = vk_to_raw_value(&src.export_from_imported_handle_types);
        dst.compatible_handle_types = vk_to_raw_value(&src.compatible_handle_types);
        dst.external_semaphore_features = vk_to_raw_value(&src.external_semaphore_features);
    }
}

impl Default for VkExternalSemaphoreProperties {
    fn default() -> VkExternalSemaphoreProperties {
        VkExternalSemaphoreProperties {
            export_from_imported_handle_types: VkExternalSemaphoreHandleTypeFlags::default(),
            compatible_handle_types: VkExternalSemaphoreHandleTypeFlags::default(),
            external_semaphore_features: VkExternalSemaphoreFeatureFlags::default(),
        }
    }
}

impl VkSetup for VkExternalSemaphoreProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkExternalSemaphoreProperties {
    fn vk_free(&mut self) {
        
    }
}