// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkViewportCoordinateSwizzle = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkViewportCoordinateSwizzle {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
    PositiveW = 6,
    NegativeW = 7,
}

impl VkType<RawVkViewportCoordinateSwizzle> for VkViewportCoordinateSwizzle {
    
    fn vk_to_raw(src: &VkViewportCoordinateSwizzle, dst: &mut RawVkViewportCoordinateSwizzle) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkViewportCoordinateSwizzle) -> VkViewportCoordinateSwizzle {
        unsafe {
            *((src as *const i32) as *const VkViewportCoordinateSwizzle)
        }
    }
    
    fn vk_default() -> VkViewportCoordinateSwizzle {
        VkViewportCoordinateSwizzle::PositiveX
    }
}