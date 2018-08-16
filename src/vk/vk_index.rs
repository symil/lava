// Copied from `scripts/static/`

use utils::vk_traits::*;

impl VkRawType<usize> for u32 {
    fn vk_to_wrapped(value: &u32) -> usize {
        *value as usize
    }
}

impl VkWrappedType<u32> for usize {
    fn vk_to_raw(value: &usize, dst: &mut u32) {
        *dst = *value as u32;
    }
}

impl VkRawType<usize> for u64 {
    fn vk_to_wrapped(value: &u64) -> usize {
        *value as usize
    }
}

impl VkWrappedType<u64> for usize {
    fn vk_to_raw(value: &usize, dst: &mut u64) {
        *dst = *value as u64;
    }
}

impl VkRawType<isize> for i32 {
    fn vk_to_wrapped(value: &i32) -> isize {
        *value as isize
    }
}

impl VkWrappedType<i32> for isize {
    fn vk_to_raw(value: &isize, dst: &mut i32) {
        *dst = *value as i32;
    }
}

impl VkRawType<isize> for i64 {
    fn vk_to_wrapped(value: &i64) -> isize {
        *value as isize
    }
}

impl VkWrappedType<i64> for isize {
    fn vk_to_raw(value: &isize, dst: &mut i64) {
        *dst = *value as i64;
    }
}