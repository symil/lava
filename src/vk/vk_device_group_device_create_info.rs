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
use vk::vk_physical_device::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupDeviceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub physical_device_count: u32,
    pub physical_devices: *mut RawVkPhysicalDevice,
}

#[derive(Debug, Clone)]
pub struct VkDeviceGroupDeviceCreateInfo<'a, 'b>
    where
        'b: 'a,
{
    pub physical_devices: &'a [&'b VkPhysicalDevice],
}

impl<'a, 'b> VkWrappedType<RawVkDeviceGroupDeviceCreateInfo> for VkDeviceGroupDeviceCreateInfo<'a, 'b>
    where
        'b: 'a,
{
    fn vk_to_raw(src: &VkDeviceGroupDeviceCreateInfo, dst: &mut RawVkDeviceGroupDeviceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupDeviceCreateInfo);
        dst.next = ptr::null();
        dst.physical_device_count = src.physical_devices.len() as u32;
        dst.physical_devices = new_ptr_vk_array_from_ref(src.physical_devices);
    }
}

impl Default for VkDeviceGroupDeviceCreateInfo<'static, 'static> {
    fn default() -> VkDeviceGroupDeviceCreateInfo<'static, 'static> {
        VkDeviceGroupDeviceCreateInfo {
            physical_devices: &[],
        }
    }
}

impl<'a, 'b> VkSetup for VkDeviceGroupDeviceCreateInfo<'a, 'b>
    where
        'b: 'a,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGroupDeviceCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.physical_devices);
    }
}