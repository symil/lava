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
use vk::amd::vk_rasterization_order::*;

#[repr(C)]
pub struct RawVkPipelineRasterizationStateRasterizationOrder {
    s_type: RawVkStructureType,
    next: *const c_void,
    rasterization_order: RawVkRasterizationOrder,
}

#[derive(Debug, Clone)]
pub struct VkPipelineRasterizationStateRasterizationOrder {
    pub rasterization_order: VkRasterizationOrder,
}

impl VkRawType<VkPipelineRasterizationStateRasterizationOrder> for RawVkPipelineRasterizationStateRasterizationOrder {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationStateRasterizationOrder) -> VkPipelineRasterizationStateRasterizationOrder {
        VkPipelineRasterizationStateRasterizationOrder {
            rasterization_order: RawVkRasterizationOrder::vk_to_wrapped(&src.rasterization_order),
        }
    }
}

impl VkWrappedType<RawVkPipelineRasterizationStateRasterizationOrder> for VkPipelineRasterizationStateRasterizationOrder {
    fn vk_to_raw(src: &VkPipelineRasterizationStateRasterizationOrder, dst: &mut RawVkPipelineRasterizationStateRasterizationOrder) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineRasterizationStateRasterizationOrderAmd);
        dst.next = ptr::null();
        dst.rasterization_order = vk_to_raw_value(&src.rasterization_order);
    }
}

impl Default for VkPipelineRasterizationStateRasterizationOrder {
    fn default() -> VkPipelineRasterizationStateRasterizationOrder {
        VkPipelineRasterizationStateRasterizationOrder {
            rasterization_order: VkRasterizationOrder::default(),
        }
    }
}

impl VkSetup for VkPipelineRasterizationStateRasterizationOrder {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineRasterizationStateRasterizationOrder {
    fn vk_free(&mut self) {
        
    }
}