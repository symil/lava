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
use vk::vk_sample_count_flags::*;
use vk::vk_extent_2d::*;

#[repr(C)]
pub struct RawVkPhysicalDeviceSampleLocationsProperties {
    s_type: RawVkStructureType,
    next: *const c_void,
    sample_location_sample_counts: RawVkSampleCountFlags,
    max_sample_location_grid_size: RawVkExtent2D,
    sample_location_coordinate_range: [f32; 2],
    sample_location_sub_pixel_bits: u32,
    variable_sample_locations: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSampleLocationsProperties {
    pub sample_location_sample_counts: VkSampleCountFlags,
    pub max_sample_location_grid_size: VkExtent2D,
    pub sample_location_coordinate_range: [f32; 2],
    pub sample_location_sub_pixel_bits: usize,
    pub variable_sample_locations: bool,
}

impl VkRawType<VkPhysicalDeviceSampleLocationsProperties> for RawVkPhysicalDeviceSampleLocationsProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceSampleLocationsProperties) -> VkPhysicalDeviceSampleLocationsProperties {
        VkPhysicalDeviceSampleLocationsProperties {
            sample_location_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.sample_location_sample_counts),
            max_sample_location_grid_size: RawVkExtent2D::vk_to_wrapped(&src.max_sample_location_grid_size),
            sample_location_coordinate_range: unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.sample_location_coordinate_range, &mut dst_array); dst_array },
            sample_location_sub_pixel_bits: u32::vk_to_wrapped(&src.sample_location_sub_pixel_bits),
            variable_sample_locations: u32::vk_to_wrapped(&src.variable_sample_locations),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceSampleLocationsProperties> for VkPhysicalDeviceSampleLocationsProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceSampleLocationsProperties, dst: &mut RawVkPhysicalDeviceSampleLocationsProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSampleLocationsPropertiesExt);
        dst.next = ptr::null();
        dst.sample_location_sample_counts = vk_to_raw_value(&src.sample_location_sample_counts);
        dst.max_sample_location_grid_size = vk_to_raw_value(&src.max_sample_location_grid_size);
        dst.sample_location_coordinate_range = unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.sample_location_coordinate_range, &mut dst_array); dst_array };
        dst.sample_location_sub_pixel_bits = vk_to_raw_value(&src.sample_location_sub_pixel_bits);
        dst.variable_sample_locations = vk_to_raw_value(&src.variable_sample_locations);
    }
}

impl Default for VkPhysicalDeviceSampleLocationsProperties {
    fn default() -> VkPhysicalDeviceSampleLocationsProperties {
        VkPhysicalDeviceSampleLocationsProperties {
            sample_location_sample_counts: VkSampleCountFlags::default(),
            max_sample_location_grid_size: VkExtent2D::default(),
            sample_location_coordinate_range: [0.0; 2],
            sample_location_sub_pixel_bits: 0,
            variable_sample_locations: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceSampleLocationsProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.max_sample_location_grid_size, fn_table, instance, device);
    }
}

impl VkFree for RawVkPhysicalDeviceSampleLocationsProperties {
    fn vk_free(&mut self) {
        RawVkExtent2D::vk_free(&mut self.max_sample_location_grid_size);
    }
}