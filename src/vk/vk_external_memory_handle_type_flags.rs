// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkExternalMemoryHandleTypeFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkExternalMemoryHandleTypeFlags {
    opaque_fd: bool,
    opaque_win32: bool,
    opaque_win32_kmt: bool,
    d3d11_texture: bool,
    d3d11_texture_kmt: bool,
    d3d12_heap: bool,
    d3d12_resource: bool,
    dma_buf_ext: bool,
    android_hardware_buffer_android: bool,
    host_allocation_ext: bool,
    host_mapped_foreign_memory_ext: bool,
}

impl VkRawType<VkExternalMemoryHandleTypeFlags> for RawVkExternalMemoryHandleTypeFlags {
    fn vk_to_wrapped(src: &RawVkExternalMemoryHandleTypeFlags) -> VkExternalMemoryHandleTypeFlags {
        VkExternalMemoryHandleTypeFlags {
            opaque_fd: (src & 0x00000001) != 0,
            opaque_win32: (src & 0x00000002) != 0,
            opaque_win32_kmt: (src & 0x00000004) != 0,
            d3d11_texture: (src & 0x00000008) != 0,
            d3d11_texture_kmt: (src & 0x00000010) != 0,
            d3d12_heap: (src & 0x00000020) != 0,
            d3d12_resource: (src & 0x00000040) != 0,
            dma_buf_ext: (src & 0x00000200) != 0,
            android_hardware_buffer_android: (src & 0x00000400) != 0,
            host_allocation_ext: (src & 0x00000080) != 0,
            host_mapped_foreign_memory_ext: (src & 0x00000100) != 0,
        }
    }
}

impl VkWrappedType<RawVkExternalMemoryHandleTypeFlags> for VkExternalMemoryHandleTypeFlags {
    fn vk_to_raw(src: &VkExternalMemoryHandleTypeFlags, dst: &mut RawVkExternalMemoryHandleTypeFlags) {
        *dst = 0;
        if src.opaque_fd { *dst |= 0x00000001; }
        if src.opaque_win32 { *dst |= 0x00000002; }
        if src.opaque_win32_kmt { *dst |= 0x00000004; }
        if src.d3d11_texture { *dst |= 0x00000008; }
        if src.d3d11_texture_kmt { *dst |= 0x00000010; }
        if src.d3d12_heap { *dst |= 0x00000020; }
        if src.d3d12_resource { *dst |= 0x00000040; }
        if src.dma_buf_ext { *dst |= 0x00000200; }
        if src.android_hardware_buffer_android { *dst |= 0x00000400; }
        if src.host_allocation_ext { *dst |= 0x00000080; }
        if src.host_mapped_foreign_memory_ext { *dst |= 0x00000100; }
    }
}

impl VkDefault for VkExternalMemoryHandleTypeFlags {
    fn vk_default() -> VkExternalMemoryHandleTypeFlags {
        VkExternalMemoryHandleTypeFlags {
            opaque_fd: false,
            opaque_win32: false,
            opaque_win32_kmt: false,
            d3d11_texture: false,
            d3d11_texture_kmt: false,
            d3d12_heap: false,
            d3d12_resource: false,
            dma_buf_ext: false,
            android_hardware_buffer_android: false,
            host_allocation_ext: false,
            host_mapped_foreign_memory_ext: false,
        }
    }
}