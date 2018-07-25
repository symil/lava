// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkRasterizationOrder = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkRasterizationOrder {
    Strict = 0,
    Relaxed = 1,
}

impl VkType<RawVkRasterizationOrder> for VkRasterizationOrder {
    
    fn vk_to_raw(src: &VkRasterizationOrder, dst: &mut RawVkRasterizationOrder) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkRasterizationOrder) -> VkRasterizationOrder {
        unsafe {
            *((src as *const i32) as *const VkRasterizationOrder)
        }
    }
    
    fn vk_default() -> VkRasterizationOrder {
        VkRasterizationOrder::Strict
    }
}