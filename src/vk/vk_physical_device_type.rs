use std::convert::From;
use std::fmt::*;

pub type RawVkPhysicalDeviceType = i32;

#[derive(Debug)]
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

impl Display for VkPhysicalDeviceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            VkPhysicalDeviceType::Other => write!(f, "Other"),
            VkPhysicalDeviceType::IntegratedGpu => write!(f, "IntegratedGpu"),
            VkPhysicalDeviceType::DiscreteGpu => write!(f, "DiscreteGpu"),
            VkPhysicalDeviceType::VirtualGpu => write!(f, "VirtualGpu"),
            VkPhysicalDeviceType::Cpu => write!(f, "Cpu")
        }
    }
}