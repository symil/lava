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

/// Wrapper for [VkPhysicalDevicePCIBusInfoPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDevicePCIBusInfoProperties {
    pub pci_domain: usize,
    pub pci_bus: usize,
    pub pci_device: usize,
    pub pci_function: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDevicePCIBusInfoProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub pci_domain: u32,
    pub pci_bus: u32,
    pub pci_device: u32,
    pub pci_function: u32,
}

impl VkWrappedType<RawVkPhysicalDevicePCIBusInfoProperties> for VkPhysicalDevicePCIBusInfoProperties {
    fn vk_to_raw(src: &VkPhysicalDevicePCIBusInfoProperties, dst: &mut RawVkPhysicalDevicePCIBusInfoProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDevicePciBusInfoPropertiesExt);
        dst.next = ptr::null_mut();
        dst.pci_domain = vk_to_raw_value(&src.pci_domain);
        dst.pci_bus = vk_to_raw_value(&src.pci_bus);
        dst.pci_device = vk_to_raw_value(&src.pci_device);
        dst.pci_function = vk_to_raw_value(&src.pci_function);
    }
}

impl VkRawType<VkPhysicalDevicePCIBusInfoProperties> for RawVkPhysicalDevicePCIBusInfoProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDevicePCIBusInfoProperties) -> VkPhysicalDevicePCIBusInfoProperties {
        VkPhysicalDevicePCIBusInfoProperties {
            pci_domain: u32::vk_to_wrapped(&src.pci_domain),
            pci_bus: u32::vk_to_wrapped(&src.pci_bus),
            pci_device: u32::vk_to_wrapped(&src.pci_device),
            pci_function: u32::vk_to_wrapped(&src.pci_function),
        }
    }
}

impl Default for VkPhysicalDevicePCIBusInfoProperties {
    fn default() -> VkPhysicalDevicePCIBusInfoProperties {
        VkPhysicalDevicePCIBusInfoProperties {
            pci_domain: 0,
            pci_bus: 0,
            pci_device: 0,
            pci_function: 0,
        }
    }
}

impl VkSetup for VkPhysicalDevicePCIBusInfoProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDevicePCIBusInfoProperties {
    fn vk_free(&self) {
        
    }
}