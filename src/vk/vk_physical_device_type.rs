// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPhysicalDeviceType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}

impl VkRawType<VkPhysicalDeviceType> for RawVkPhysicalDeviceType {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceType) -> VkPhysicalDeviceType {
        unsafe {
            *((src as *const i32) as *const VkPhysicalDeviceType)
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceType> for VkPhysicalDeviceType {
    fn vk_to_raw(src: &VkPhysicalDeviceType, dst: &mut RawVkPhysicalDeviceType) {
        *dst = *src as i32
    }
}

impl VkDefault for VkPhysicalDeviceType {
    fn vk_default() -> VkPhysicalDeviceType {
        VkPhysicalDeviceType::Other
    }
}