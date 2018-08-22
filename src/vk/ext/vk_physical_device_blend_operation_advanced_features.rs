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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceBlendOperationAdvancedFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub advanced_blend_coherent_operations: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeatures {
    pub advanced_blend_coherent_operations: bool,
}

impl VkRawType<VkPhysicalDeviceBlendOperationAdvancedFeatures> for RawVkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceBlendOperationAdvancedFeatures) -> VkPhysicalDeviceBlendOperationAdvancedFeatures {
        VkPhysicalDeviceBlendOperationAdvancedFeatures {
            advanced_blend_coherent_operations: u32::vk_to_wrapped(&src.advanced_blend_coherent_operations),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceBlendOperationAdvancedFeatures> for VkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceBlendOperationAdvancedFeatures, dst: &mut RawVkPhysicalDeviceBlendOperationAdvancedFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceBlendOperationAdvancedFeaturesExt);
        dst.next = ptr::null();
        dst.advanced_blend_coherent_operations = vk_to_raw_value(&src.advanced_blend_coherent_operations);
    }
}

impl Default for VkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn default() -> VkPhysicalDeviceBlendOperationAdvancedFeatures {
        VkPhysicalDeviceBlendOperationAdvancedFeatures {
            advanced_blend_coherent_operations: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceBlendOperationAdvancedFeatures {
    fn vk_free(&mut self) {
        
    }
}