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
use vulkan::vk::{VkImageSubresourceLayers,RawVkImageSubresourceLayers};
use vulkan::vk::{VkOffset3D,RawVkOffset3D};

/// Wrapper for [VkImageBlit](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageBlit.html).
#[derive(Debug, Clone)]
pub struct VkImageBlit {
    pub src_subresource: VkImageSubresourceLayers,
    pub src_offsets: [VkOffset3D; 2],
    pub dst_subresource: VkImageSubresourceLayers,
    pub dst_offsets: [VkOffset3D; 2],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageBlit {
    pub src_subresource: RawVkImageSubresourceLayers,
    pub src_offsets: [RawVkOffset3D; 2],
    pub dst_subresource: RawVkImageSubresourceLayers,
    pub dst_offsets: [RawVkOffset3D; 2],
}

impl VkWrappedType<RawVkImageBlit> for VkImageBlit {
    fn vk_to_raw(src: &VkImageBlit, dst: &mut RawVkImageBlit) {
        dst.src_subresource = vk_to_raw_value(&src.src_subresource);
        dst.src_offsets = unsafe { let mut dst_array : [RawVkOffset3D; 2] = mem::uninitialized(); vk_to_raw_array(&src.src_offsets, &mut dst_array); dst_array };
        dst.dst_subresource = vk_to_raw_value(&src.dst_subresource);
        dst.dst_offsets = unsafe { let mut dst_array : [RawVkOffset3D; 2] = mem::uninitialized(); vk_to_raw_array(&src.dst_offsets, &mut dst_array); dst_array };
    }
}

impl VkRawType<VkImageBlit> for RawVkImageBlit {
    fn vk_to_wrapped(src: &RawVkImageBlit) -> VkImageBlit {
        VkImageBlit {
            src_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.src_subresource),
            src_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); vk_to_wrapped_array(&src.src_offsets, &mut dst_array); dst_array },
            dst_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.dst_subresource),
            dst_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); vk_to_wrapped_array(&src.dst_offsets, &mut dst_array); dst_array },
        }
    }
}

impl Default for VkImageBlit {
    fn default() -> VkImageBlit {
        VkImageBlit {
            src_subresource: Default::default(),
            src_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
            dst_subresource: Default::default(),
            dst_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
        }
    }
}

impl VkSetup for VkImageBlit {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.src_subresource, fn_table);
        VkSetup::vk_setup(&mut self.dst_subresource, fn_table);
    }
}

impl VkFree for RawVkImageBlit {
    fn vk_free(&self) {
        for elt in self.src_offsets.iter() { RawVkOffset3D::vk_free(elt); };
        for elt in self.dst_offsets.iter() { RawVkOffset3D::vk_free(elt); };
    }
}