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
use vk::nvx::vk_object_entry_type::*;
use vk::nvx::vk_object_entry_usage_flags::*;
use vk::vk_buffer::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkObjectTableVertexBufferEntry {
    pub type_: RawVkObjectEntryType,
    pub flags: RawVkObjectEntryUsageFlags,
    pub buffer: RawVkBuffer,
}

#[derive(Debug, Clone)]
pub struct VkObjectTableVertexBufferEntry<'a> {
    pub type_: VkObjectEntryType,
    pub flags: VkObjectEntryUsageFlags,
    pub buffer: &'a VkBuffer,
}

impl<'a> VkWrappedType<RawVkObjectTableVertexBufferEntry> for VkObjectTableVertexBufferEntry<'a> {
    fn vk_to_raw(src: &VkObjectTableVertexBufferEntry, dst: &mut RawVkObjectTableVertexBufferEntry) {
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.buffer = vk_to_raw_value(src.buffer);
    }
}

impl Default for VkObjectTableVertexBufferEntry<'static> {
    fn default() -> VkObjectTableVertexBufferEntry<'static> {
        VkObjectTableVertexBufferEntry {
            type_: VkObjectEntryType::default(),
            flags: VkObjectEntryUsageFlags::default(),
            buffer: vk_null_ref(),
        }
    }
}

impl<'a> VkSetup for VkObjectTableVertexBufferEntry<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkObjectTableVertexBufferEntry {
    fn vk_free(&mut self) {
        
    }
}