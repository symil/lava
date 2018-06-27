// Generated by `scripts/generate_type.js`

use std::convert::From;
use vk::*;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RawVkMemoryHeap {
    size: u64,
    flags: RawVkMemoryHeapFlags
}

#[derive(Debug)]
pub struct VkMemoryHeap {
    pub size: u64,
    pub flags: VkMemoryHeapFlags
}

impl<'a> From<&'a RawVkMemoryHeap> for VkMemoryHeap {
    fn from(value: &'a RawVkMemoryHeap) -> Self {
        VkMemoryHeap {
            size: value.size,
            flags: VkMemoryHeapFlags::from(&value.flags)
        }
    }
}

impl<'a> From<&'a VkMemoryHeap> for RawVkMemoryHeap {
    fn from(value: &'a VkMemoryHeap) -> Self {
        RawVkMemoryHeap {
            size: value.size,
            flags: RawVkMemoryHeapFlags::from(&value.flags)
        }
    }
}

