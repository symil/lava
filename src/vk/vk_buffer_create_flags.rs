// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkBufferCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkBufferCreateFlags {
    sparse_binding: bool,
    sparse_residency: bool,
    sparse_aliased: bool,
    protected: bool,
}

impl VkRawType<VkBufferCreateFlags> for RawVkBufferCreateFlags {
    fn vk_to_wrapped(src: &RawVkBufferCreateFlags) -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: (src & 0x00000001) != 0,
            sparse_residency: (src & 0x00000002) != 0,
            sparse_aliased: (src & 0x00000004) != 0,
            protected: (src & 0x00000008) != 0,
        }
    }
}

impl VkWrappedType<RawVkBufferCreateFlags> for VkBufferCreateFlags {
    fn vk_to_raw(src: &VkBufferCreateFlags, dst: &mut RawVkBufferCreateFlags) {
        *dst = 0;
        if src.sparse_binding { *dst |= 0x00000001; }
        if src.sparse_residency { *dst |= 0x00000002; }
        if src.sparse_aliased { *dst |= 0x00000004; }
        if src.protected { *dst |= 0x00000008; }
    }
}

pub static STATIC_VK_BUFFER_CREATE_FLAGS : VkBufferCreateFlags = VkBufferCreateFlags {
    sparse_binding: false,
    sparse_residency: false,
    sparse_aliased: false,
    protected: false,
};

impl VkDefault for VkBufferCreateFlags {
    fn vk_default() -> VkBufferCreateFlags {
        STATIC_VK_BUFFER_CREATE_FLAGS
    }
}