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
use vk::vk_image::*;
use vk::vk_sparse_image_memory_bind::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageMemoryBindInfo {
    pub image: RawVkImage,
    pub bind_count: u32,
    pub binds: *mut RawVkSparseImageMemoryBind,
}

#[derive(Debug, Clone)]
pub struct VkSparseImageMemoryBindInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    pub image: &'a VkImage,
    pub binds: &'b [VkSparseImageMemoryBind<'c>],
}

impl<'a, 'b, 'c> VkWrappedType<RawVkSparseImageMemoryBindInfo> for VkSparseImageMemoryBindInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    fn vk_to_raw(src: &VkSparseImageMemoryBindInfo, dst: &mut RawVkSparseImageMemoryBindInfo) {
        dst.image = vk_to_raw_value(src.image);
        dst.bind_count = src.binds.len() as u32;
        dst.binds = new_ptr_vk_array(src.binds);
    }
}

impl Default for VkSparseImageMemoryBindInfo<'static, 'static, 'static> {
    fn default() -> VkSparseImageMemoryBindInfo<'static, 'static, 'static> {
        VkSparseImageMemoryBindInfo {
            image: vk_null_ref(),
            binds: &[],
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkSparseImageMemoryBindInfo<'a, 'b, 'c>
    where
        'c: 'b,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSparseImageMemoryBindInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.bind_count as usize, self.binds);
    }
}