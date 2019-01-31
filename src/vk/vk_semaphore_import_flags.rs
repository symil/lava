// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreImportFlags {
    pub temporary: bool,
}

pub type RawVkSemaphoreImportFlags = u32;

impl VkWrappedType<RawVkSemaphoreImportFlags> for VkSemaphoreImportFlags {
    fn vk_to_raw(src: &VkSemaphoreImportFlags, dst: &mut RawVkSemaphoreImportFlags) {
        *dst = 0;
        if src.temporary { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkSemaphoreImportFlags> for RawVkSemaphoreImportFlags {
    fn vk_to_wrapped(src: &RawVkSemaphoreImportFlags) -> VkSemaphoreImportFlags {
        VkSemaphoreImportFlags {
            temporary: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkSemaphoreImportFlags {
    fn default() -> VkSemaphoreImportFlags {
        VkSemaphoreImportFlags {
            temporary: false,
        }
    }
}

impl VkSemaphoreImportFlags {
    
    pub fn none() -> VkSemaphoreImportFlags {
        VkSemaphoreImportFlags {
            temporary: false,
        }
    }
    
    pub fn all() -> VkSemaphoreImportFlags {
        VkSemaphoreImportFlags {
            temporary: true,
        }
    }
}

#[macro_export]
macro_rules! VkSemaphoreImportFlags {
    ( $( $x:ident ),* ) => {
        VkSemaphoreImportFlags {
            $($x: true,)*
            ..VkSemaphoreImportFlags::none()
        }
    }
}

impl VkSemaphoreImportFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.temporary { 0x00000001 } else { 0 }
    }
}