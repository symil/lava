// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkBuildAccelerationStructureFlags {
    pub allow_update: bool,
    pub allow_compaction: bool,
    pub prefer_fast_trace: bool,
    pub prefer_fast_build: bool,
    pub low_memory: bool,
}

pub type RawVkBuildAccelerationStructureFlags = u32;

impl VkWrappedType<RawVkBuildAccelerationStructureFlags> for VkBuildAccelerationStructureFlags {
    fn vk_to_raw(src: &VkBuildAccelerationStructureFlags, dst: &mut RawVkBuildAccelerationStructureFlags) {
        *dst = 0;
        if src.allow_update { *dst |= 0x00000001; }
        if src.allow_compaction { *dst |= 0x00000002; }
        if src.prefer_fast_trace { *dst |= 0x00000004; }
        if src.prefer_fast_build { *dst |= 0x00000008; }
        if src.low_memory { *dst |= 0x00000010; }
    }
}

impl VkRawType<VkBuildAccelerationStructureFlags> for RawVkBuildAccelerationStructureFlags {
    fn vk_to_wrapped(src: &RawVkBuildAccelerationStructureFlags) -> VkBuildAccelerationStructureFlags {
        VkBuildAccelerationStructureFlags {
            allow_update: (src & 0x00000001) != 0,
            allow_compaction: (src & 0x00000002) != 0,
            prefer_fast_trace: (src & 0x00000004) != 0,
            prefer_fast_build: (src & 0x00000008) != 0,
            low_memory: (src & 0x00000010) != 0,
        }
    }
}

impl Default for VkBuildAccelerationStructureFlags {
    fn default() -> VkBuildAccelerationStructureFlags {
        VkBuildAccelerationStructureFlags {
            allow_update: false,
            allow_compaction: false,
            prefer_fast_trace: false,
            prefer_fast_build: false,
            low_memory: false,
        }
    }
}

impl VkBuildAccelerationStructureFlags {
    
    pub fn none() -> VkBuildAccelerationStructureFlags {
        VkBuildAccelerationStructureFlags {
            allow_update: false,
            allow_compaction: false,
            prefer_fast_trace: false,
            prefer_fast_build: false,
            low_memory: false,
        }
    }
    
    pub fn all() -> VkBuildAccelerationStructureFlags {
        VkBuildAccelerationStructureFlags {
            allow_update: true,
            allow_compaction: true,
            prefer_fast_trace: true,
            prefer_fast_build: true,
            low_memory: true,
        }
    }
}

#[macro_export]
macro_rules! VkBuildAccelerationStructureFlags {
    ( $( $x:ident ),* ) => {
        VkBuildAccelerationStructureFlags {
            $($x: true,)*
            ..VkBuildAccelerationStructureFlags::none()
        }
    }
}

impl VkBuildAccelerationStructureFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.allow_update { 0x00000001 } else { 0 }
        + if self.allow_compaction { 0x00000002 } else { 0 }
        + if self.prefer_fast_trace { 0x00000004 } else { 0 }
        + if self.prefer_fast_build { 0x00000008 } else { 0 }
        + if self.low_memory { 0x00000010 } else { 0 }
    }
}