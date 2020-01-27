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
use vulkan::vk::{VkImage,RawVkImage};
use vulkan::vk::{VkSparseImageMemoryBind,RawVkSparseImageMemoryBind};

/// Wrapper for [VkSparseImageMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBindInfo.html).
#[derive(Debug, Clone)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub binds: Vec<VkSparseImageMemoryBind>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageMemoryBindInfo {
    pub image: RawVkImage,
    pub bind_count: u32,
    pub binds: *mut RawVkSparseImageMemoryBind,
}

impl VkWrappedType<RawVkSparseImageMemoryBindInfo> for VkSparseImageMemoryBindInfo {
    fn vk_to_raw(src: &VkSparseImageMemoryBindInfo, dst: &mut RawVkSparseImageMemoryBindInfo) {
        dst.image = vk_to_raw_value(&src.image);
        dst.bind_count = src.binds.len() as u32;
        dst.binds = new_ptr_vk_array(&src.binds);
    }
}

impl VkRawType<VkSparseImageMemoryBindInfo> for RawVkSparseImageMemoryBindInfo {
    fn vk_to_wrapped(src: &RawVkSparseImageMemoryBindInfo) -> VkSparseImageMemoryBindInfo {
        VkSparseImageMemoryBindInfo {
            image: RawVkImage::vk_to_wrapped(&src.image),
            binds: new_vk_array(src.bind_count, src.binds),
        }
    }
}

impl Default for VkSparseImageMemoryBindInfo {
    fn default() -> VkSparseImageMemoryBindInfo {
        VkSparseImageMemoryBindInfo {
            image: Default::default(),
            binds: Vec::new(),
        }
    }
}

impl VkSetup for VkSparseImageMemoryBindInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.image, fn_table);
    }
}

impl VkFree for RawVkSparseImageMemoryBindInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.bind_count as usize, self.binds);
    }
}