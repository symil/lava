use std::convert::From;

pub type RawVkPhysicalDeviceType = i32;

#[derive(PartialEq)]
pub enum VkPhysicalDeviceType {
    Other,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu
}

impl<'a> From<&'a RawVkPhysicalDeviceType> for VkPhysicalDeviceType {
    fn from(value: &'a RawVkPhysicalDeviceType) -> Self {
        match value {
            0 => VkPhysicalDeviceType::Other,
            1 => VkPhysicalDeviceType::IntegratedGpu,
            2 => VkPhysicalDeviceType::DiscreteGpu,
            3 => VkPhysicalDeviceType::VirtualGpu,
            4 => VkPhysicalDeviceType::Cpu,
            _ => panic!("Vulkan wrapper error: unable to convert int32 {} into VkPhysicalDeviceType value", value)
        }
    }
}