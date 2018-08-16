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
use vk::khr::vk_display_plane_properties::*;

#[repr(C)]
pub struct RawVkDisplayPlaneProperties2 {
    s_type: RawVkStructureType,
    next: *const c_void,
    display_plane_properties: RawVkDisplayPlaneProperties,
}

#[derive(Debug, Clone)]
pub struct VkDisplayPlaneProperties2 {
    pub display_plane_properties: VkDisplayPlaneProperties,
}

impl VkRawType<VkDisplayPlaneProperties2> for RawVkDisplayPlaneProperties2 {
    fn vk_to_wrapped(src: &RawVkDisplayPlaneProperties2) -> VkDisplayPlaneProperties2 {
        VkDisplayPlaneProperties2 {
            display_plane_properties: RawVkDisplayPlaneProperties::vk_to_wrapped(&src.display_plane_properties),
        }
    }
}

impl VkSetup for VkDisplayPlaneProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.display_plane_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayPlaneProperties2 {
    fn vk_free(&mut self) {
        RawVkDisplayPlaneProperties::vk_free(&mut self.display_plane_properties);
    }
}