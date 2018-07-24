use utils::vk_type::VkType;

impl VkType<u32> for bool {
    fn vk_to_raw(value: &bool, dst: &mut u32) {
        *dst = if *value { 1 } else { 0 }
    }

    fn vk_from_raw(value: &u32) -> bool {
        *value != 0
    }

    fn vk_default() -> bool {
        false
    }
}