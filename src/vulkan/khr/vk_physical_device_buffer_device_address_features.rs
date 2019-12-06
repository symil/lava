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

/// Wrapper for [VkPhysicalDeviceBufferDeviceAddressFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesKHR.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
    pub buffer_device_address: bool,
    pub buffer_device_address_capture_replay: bool,
    pub buffer_device_address_multi_device: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub buffer_device_address: u32,
    pub buffer_device_address_capture_replay: u32,
    pub buffer_device_address_multi_device: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceBufferDeviceAddressFeatures> for VkPhysicalDeviceBufferDeviceAddressFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceBufferDeviceAddressFeatures, dst: &mut RawVkPhysicalDeviceBufferDeviceAddressFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceBufferDeviceAddressFeaturesKhr);
        dst.next = ptr::null_mut();
        dst.buffer_device_address = vk_to_raw_value(&src.buffer_device_address);
        dst.buffer_device_address_capture_replay = vk_to_raw_value(&src.buffer_device_address_capture_replay);
        dst.buffer_device_address_multi_device = vk_to_raw_value(&src.buffer_device_address_multi_device);
    }
}

impl VkRawType<VkPhysicalDeviceBufferDeviceAddressFeatures> for RawVkPhysicalDeviceBufferDeviceAddressFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceBufferDeviceAddressFeatures) -> VkPhysicalDeviceBufferDeviceAddressFeatures {
        VkPhysicalDeviceBufferDeviceAddressFeatures {
            buffer_device_address: u32::vk_to_wrapped(&src.buffer_device_address),
            buffer_device_address_capture_replay: u32::vk_to_wrapped(&src.buffer_device_address_capture_replay),
            buffer_device_address_multi_device: u32::vk_to_wrapped(&src.buffer_device_address_multi_device),
        }
    }
}

impl Default for VkPhysicalDeviceBufferDeviceAddressFeatures {
    fn default() -> VkPhysicalDeviceBufferDeviceAddressFeatures {
        VkPhysicalDeviceBufferDeviceAddressFeatures {
            buffer_device_address: false,
            buffer_device_address_capture_replay: false,
            buffer_device_address_multi_device: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceBufferDeviceAddressFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceBufferDeviceAddressFeatures {
    fn vk_free(&self) {
        
    }
}