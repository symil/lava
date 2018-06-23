use std::convert::From;
use vk_types::vk_physical_device_type::*;
use std::string::String;
use std::ffi::CStr;
use vk_types::vk_physical_device_limits::*;
use vk_types::vk_physical_device_sparse_properties::*;

#[repr(C)]
pub struct RawVkPhysicalDeviceProperties {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: RawVkPhysicalDeviceType,
    device_name: [u8; 256],
    pipeline_cache_uuid: [u8; 16],
    limits: RawVkPhysicalDeviceLimits,
    sparse_properties: RawVkPhysicalDeviceSparseProperties
}

#[derive(Debug)]
pub struct VkPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: VkPhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties
}

impl<'a> From<&'a RawVkPhysicalDeviceProperties> for VkPhysicalDeviceProperties {
    fn from(value: &'a RawVkPhysicalDeviceProperties) -> Self {
        VkPhysicalDeviceProperties {
            api_version: value.api_version,
            driver_version: value.driver_version,
            vendor_id: value.vendor_id,
            device_id: value.device_id,
            device_type: VkPhysicalDeviceType::from(&value.device_type),
            device_name: unsafe { String::from_utf8_unchecked((&value.device_name).to_vec().into_iter().filter(|x| *x != 0).collect()) },
            pipeline_cache_uuid: value.pipeline_cache_uuid,
            limits: VkPhysicalDeviceLimits::from(&value.limits),
            sparse_properties: VkPhysicalDeviceSparseProperties::from(&value.sparse_properties)
        }
    }
}