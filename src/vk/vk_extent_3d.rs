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

#[repr(C)]
pub struct RawVkExtent3D {
    width: u32,
    height: u32,
    depth: u32,
}

#[derive(Debug, Clone)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: usize,
}

impl VkRawType<VkExtent3D> for RawVkExtent3D {
    fn vk_to_wrapped(src: &RawVkExtent3D) -> VkExtent3D {
        VkExtent3D {
            width: src.width,
            height: src.height,
            depth: u32::vk_to_wrapped(&src.depth),
        }
    }
}

impl VkWrappedType<RawVkExtent3D> for VkExtent3D {
    fn vk_to_raw(src: &VkExtent3D, dst: &mut RawVkExtent3D) {
        dst.width = src.width;
        dst.height = src.height;
        dst.depth = vk_to_raw_value(&src.depth);
    }
}

impl Default for VkExtent3D {
    fn default() -> VkExtent3D {
        VkExtent3D {
            width: 0,
            height: 0,
            depth: 0,
        }
    }
}

impl VkSetup for VkExtent3D {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkExtent3D {
    fn vk_free(&mut self) {
        
    }
}