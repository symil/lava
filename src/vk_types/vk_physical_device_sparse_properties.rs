use std::convert::From;

#[repr(C)]
pub struct RawVkPhysicalDeviceSparseProperties {
    residency_standard2_dblock_shape: u32,
    residency_standard2_dmultisample_block_shape: u32,
    residency_standard3_dblock_shape: u32,
    residency_aligned_mip_size: u32,
    residency_non_resident_strict: u32
}

pub struct VkPhysicalDeviceSparseProperties {
    pub residency_standard2_dblock_shape: bool,
    pub residency_standard2_dmultisample_block_shape: bool,
    pub residency_standard3_dblock_shape: bool,
    pub residency_aligned_mip_size: bool,
    pub residency_non_resident_strict: bool
}

impl<'a> From<&'a RawVkPhysicalDeviceSparseProperties> for VkPhysicalDeviceSparseProperties {
    fn from(value: &'a RawVkPhysicalDeviceSparseProperties) -> Self {
        VkPhysicalDeviceSparseProperties {
            residency_standard2_dblock_shape: value.residency_standard2_dblock_shape != 0,
            residency_standard2_dmultisample_block_shape: value.residency_standard2_dmultisample_block_shape != 0,
            residency_standard3_dblock_shape: value.residency_standard3_dblock_shape != 0,
            residency_aligned_mip_size: value.residency_aligned_mip_size != 0,
            residency_non_resident_strict: value.residency_non_resident_strict != 0
        }
    }
}