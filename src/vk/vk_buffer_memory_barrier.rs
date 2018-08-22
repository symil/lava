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
use vk::vk_access_flags::*;
use vk::vk_buffer::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBufferMemoryBarrier {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: RawVkBuffer,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct VkBufferMemoryBarrier<'a> {
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub src_queue_family_index: usize,
    pub dst_queue_family_index: usize,
    pub buffer: &'a VkBuffer,
    pub offset: usize,
    pub size: usize,
}

impl<'a> VkWrappedType<RawVkBufferMemoryBarrier> for VkBufferMemoryBarrier<'a> {
    fn vk_to_raw(src: &VkBufferMemoryBarrier, dst: &mut RawVkBufferMemoryBarrier) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferMemoryBarrier);
        dst.next = ptr::null();
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
        dst.src_queue_family_index = vk_to_raw_value(&src.src_queue_family_index);
        dst.dst_queue_family_index = vk_to_raw_value(&src.dst_queue_family_index);
        dst.buffer = vk_to_raw_value(src.buffer);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.size = vk_to_raw_value(&src.size);
    }
}

impl Default for VkBufferMemoryBarrier<'static> {
    fn default() -> VkBufferMemoryBarrier<'static> {
        VkBufferMemoryBarrier {
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            buffer: vk_null_ref(),
            offset: 0,
            size: 0,
        }
    }
}

impl<'a> VkSetup for VkBufferMemoryBarrier<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBufferMemoryBarrier {
    fn vk_free(&mut self) {
        
    }
}