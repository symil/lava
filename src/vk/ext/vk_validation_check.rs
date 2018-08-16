// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkValidationCheck = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkValidationCheck {
    All = 0,
    Shaders = 1,
}

impl VkRawType<VkValidationCheck> for RawVkValidationCheck {
    fn vk_to_wrapped(src: &RawVkValidationCheck) -> VkValidationCheck {
        unsafe {
            *((src as *const i32) as *const VkValidationCheck)
        }
    }
}

impl VkWrappedType<RawVkValidationCheck> for VkValidationCheck {
    fn vk_to_raw(src: &VkValidationCheck, dst: &mut RawVkValidationCheck) {
        *dst = *src as i32
    }
}

impl Default for VkValidationCheck {
    fn default() -> VkValidationCheck {
        VkValidationCheck::All
    }
}