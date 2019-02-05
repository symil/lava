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
use vulkan::ext::{VkXYColor,RawVkXYColor};

/// Wrapper for [VkHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkHdrMetadataEXT.html).
#[derive(Debug, Clone)]
pub struct VkHdrMetadata {
    pub display_primary_red: VkXYColor,
    pub display_primary_green: VkXYColor,
    pub display_primary_blue: VkXYColor,
    pub white_point: VkXYColor,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkHdrMetadata {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub display_primary_red: RawVkXYColor,
    pub display_primary_green: RawVkXYColor,
    pub display_primary_blue: RawVkXYColor,
    pub white_point: RawVkXYColor,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
}

impl VkWrappedType<RawVkHdrMetadata> for VkHdrMetadata {
    fn vk_to_raw(src: &VkHdrMetadata, dst: &mut RawVkHdrMetadata) {
        dst.s_type = vk_to_raw_value(&VkStructureType::HdrMetadataExt);
        dst.next = ptr::null_mut();
        dst.display_primary_red = vk_to_raw_value(&src.display_primary_red);
        dst.display_primary_green = vk_to_raw_value(&src.display_primary_green);
        dst.display_primary_blue = vk_to_raw_value(&src.display_primary_blue);
        dst.white_point = vk_to_raw_value(&src.white_point);
        dst.max_luminance = src.max_luminance;
        dst.min_luminance = src.min_luminance;
        dst.max_content_light_level = src.max_content_light_level;
        dst.max_frame_average_light_level = src.max_frame_average_light_level;
    }
}

impl VkRawType<VkHdrMetadata> for RawVkHdrMetadata {
    fn vk_to_wrapped(src: &RawVkHdrMetadata) -> VkHdrMetadata {
        VkHdrMetadata {
            display_primary_red: RawVkXYColor::vk_to_wrapped(&src.display_primary_red),
            display_primary_green: RawVkXYColor::vk_to_wrapped(&src.display_primary_green),
            display_primary_blue: RawVkXYColor::vk_to_wrapped(&src.display_primary_blue),
            white_point: RawVkXYColor::vk_to_wrapped(&src.white_point),
            max_luminance: src.max_luminance,
            min_luminance: src.min_luminance,
            max_content_light_level: src.max_content_light_level,
            max_frame_average_light_level: src.max_frame_average_light_level,
        }
    }
}

impl Default for VkHdrMetadata {
    fn default() -> VkHdrMetadata {
        VkHdrMetadata {
            display_primary_red: Default::default(),
            display_primary_green: Default::default(),
            display_primary_blue: Default::default(),
            white_point: Default::default(),
            max_luminance: 0.0,
            min_luminance: 0.0,
            max_content_light_level: 0.0,
            max_frame_average_light_level: 0.0,
        }
    }
}

impl VkSetup for VkHdrMetadata {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.display_primary_red, fn_table);
        VkSetup::vk_setup(&mut self.display_primary_green, fn_table);
        VkSetup::vk_setup(&mut self.display_primary_blue, fn_table);
        VkSetup::vk_setup(&mut self.white_point, fn_table);
    }
}

impl VkFree for RawVkHdrMetadata {
    fn vk_free(&self) {
        
    }
}