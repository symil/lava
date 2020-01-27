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
use vulkan::vk::{VkVersion};
use vulkan::vk::{VkPhysicalDeviceType,RawVkPhysicalDeviceType};
use vulkan::vk::{VkPhysicalDeviceLimits,RawVkPhysicalDeviceLimits};
use vulkan::vk::{VkPhysicalDeviceSparseProperties,RawVkPhysicalDeviceSparseProperties};

/// Wrapper for [VkPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProperties {
    pub api_version: VkVersion,
    pub driver_version: u32,
    pub vendor_id: usize,
    pub device_id: usize,
    pub device_type: VkPhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[doc(hidden)]
#[repr(C)]
pub struct RawVkPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: RawVkPhysicalDeviceType,
    pub device_name: [c_char; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: RawVkPhysicalDeviceLimits,
    pub sparse_properties: RawVkPhysicalDeviceSparseProperties,
}

impl VkRawType<VkPhysicalDeviceProperties> for RawVkPhysicalDeviceProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceProperties) -> VkPhysicalDeviceProperties {
        VkPhysicalDeviceProperties {
            api_version: u32::vk_to_wrapped(&src.api_version),
            driver_version: src.driver_version,
            vendor_id: u32::vk_to_wrapped(&src.vendor_id),
            device_id: u32::vk_to_wrapped(&src.device_id),
            device_type: RawVkPhysicalDeviceType::vk_to_wrapped(&src.device_type),
            device_name: new_string(&src.device_name[0] as *const c_char),
            pipeline_cache_uuid: unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.pipeline_cache_uuid, &mut dst_array); dst_array },
            limits: RawVkPhysicalDeviceLimits::vk_to_wrapped(&src.limits),
            sparse_properties: RawVkPhysicalDeviceSparseProperties::vk_to_wrapped(&src.sparse_properties),
        }
    }
}

impl VkSetup for VkPhysicalDeviceProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.limits, fn_table);
        VkSetup::vk_setup(&mut self.sparse_properties, fn_table);
    }
}

impl VkFree for RawVkPhysicalDeviceProperties {
    fn vk_free(&self) {
        
    }
}