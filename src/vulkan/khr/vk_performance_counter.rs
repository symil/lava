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
use vulkan::khr::{VkPerformanceCounterUnit,RawVkPerformanceCounterUnit};
use vulkan::khr::{VkPerformanceCounterScope,RawVkPerformanceCounterScope};
use vulkan::khr::{VkPerformanceCounterStorage,RawVkPerformanceCounterStorage};

/// Wrapper for [VkPerformanceCounterKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterKHR.html).
#[derive(Debug, Clone)]
pub struct VkPerformanceCounter {
    pub unit: VkPerformanceCounterUnit,
    pub scope: VkPerformanceCounterScope,
    pub storage: VkPerformanceCounterStorage,
    pub uuid: [u8; 16],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPerformanceCounter {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub unit: RawVkPerformanceCounterUnit,
    pub scope: RawVkPerformanceCounterScope,
    pub storage: RawVkPerformanceCounterStorage,
    pub uuid: [u8; 16],
}

impl VkWrappedType<RawVkPerformanceCounter> for VkPerformanceCounter {
    fn vk_to_raw(src: &VkPerformanceCounter, dst: &mut RawVkPerformanceCounter) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PerformanceCounterKhr);
        dst.next = ptr::null_mut();
        dst.unit = vk_to_raw_value(&src.unit);
        dst.scope = vk_to_raw_value(&src.scope);
        dst.storage = vk_to_raw_value(&src.storage);
        dst.uuid = unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.uuid, &mut dst_array); dst_array };
    }
}

impl VkRawType<VkPerformanceCounter> for RawVkPerformanceCounter {
    fn vk_to_wrapped(src: &RawVkPerformanceCounter) -> VkPerformanceCounter {
        VkPerformanceCounter {
            unit: RawVkPerformanceCounterUnit::vk_to_wrapped(&src.unit),
            scope: RawVkPerformanceCounterScope::vk_to_wrapped(&src.scope),
            storage: RawVkPerformanceCounterStorage::vk_to_wrapped(&src.storage),
            uuid: unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.uuid, &mut dst_array); dst_array },
        }
    }
}

impl Default for VkPerformanceCounter {
    fn default() -> VkPerformanceCounter {
        VkPerformanceCounter {
            unit: Default::default(),
            scope: Default::default(),
            storage: Default::default(),
            uuid: [0; 16],
        }
    }
}

impl VkSetup for VkPerformanceCounter {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPerformanceCounter {
    fn vk_free(&self) {
        
    }
}