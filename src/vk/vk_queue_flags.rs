use std::convert::From;

pub type RawVkQueueFlags = u32;

pub struct VkQueueFlags {
    pub Graphics,
    pub Compute,
    pub Transfer,
    pub SparseBinding,
    pub Protected
}

impl<'a> From<&'a u32> for VkQueueFlags {
    fn from(value: &'a u32) -> Self {
        VkQueueFlags {
            Graphics: (value | 0x00000001) > 0,
            Compute: (value | 0x00000002) > 0,
            Transfer: (value | 0x00000004) > 0,
            SparseBinding: (value | 0x00000008) > 0,
            Protected: (value | 0x00000010) > 0
        }
    }
}