// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkCommandPoolCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkCommandPoolCreateFlags {
    transient: bool,
    reset_command_buffer: bool,
    protected: bool,
}

impl VkRawType<VkCommandPoolCreateFlags> for RawVkCommandPoolCreateFlags {
    fn vk_to_wrapped(src: &RawVkCommandPoolCreateFlags) -> VkCommandPoolCreateFlags {
        VkCommandPoolCreateFlags {
            transient: (src & 0x00000001) != 0,
            reset_command_buffer: (src & 0x00000002) != 0,
            protected: (src & 0x00000004) != 0,
        }
    }
}

impl VkWrappedType<RawVkCommandPoolCreateFlags> for VkCommandPoolCreateFlags {
    fn vk_to_raw(src: &VkCommandPoolCreateFlags, dst: &mut RawVkCommandPoolCreateFlags) {
        *dst = 0;
        if src.transient { *dst |= 0x00000001; }
        if src.reset_command_buffer { *dst |= 0x00000002; }
        if src.protected { *dst |= 0x00000004; }
    }
}

impl VkDefault for VkCommandPoolCreateFlags {
    fn vk_default() -> VkCommandPoolCreateFlags {
        VkCommandPoolCreateFlags {
            transient: false,
            reset_command_buffer: false,
            protected: false,
        }
    }
}