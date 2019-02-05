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
use vulkan::vk::{VkShaderModuleCreateFlags,RawVkShaderModuleCreateFlags};

/// Wrapper for [VkShaderModuleCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkShaderModuleCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkShaderModuleCreateInfo<'a> {
    pub flags: VkShaderModuleCreateFlags,
    pub code: &'a [u8],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkShaderModuleCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkShaderModuleCreateFlags,
    pub code_size: usize,
    pub code: *const u8,
}

impl<'a> VkWrappedType<RawVkShaderModuleCreateInfo> for VkShaderModuleCreateInfo<'a> {
    fn vk_to_raw(src: &VkShaderModuleCreateInfo, dst: &mut RawVkShaderModuleCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ShaderModuleCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.code_size = src.code.len();
        dst.code = src.code.as_ptr();
    }
}

impl Default for VkShaderModuleCreateInfo<'static> {
    fn default() -> VkShaderModuleCreateInfo<'static> {
        VkShaderModuleCreateInfo {
            flags: VkShaderModuleCreateFlags::default(),
            code: &[],
        }
    }
}

impl<'a> VkSetup for VkShaderModuleCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkShaderModuleCreateInfo {
    fn vk_free(&mut self) {
        
    }
}