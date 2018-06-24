use std::convert::From;

#[repr(C)]
pub struct RawVkExtent3D {
    width: u32,
    height: u32,
    depth: u32
}

#[derive(Debug)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

impl<'a> From<&'a RawVkExtent3D> for VkExtent3D {
    fn from(value: &'a RawVkExtent3D) -> Self {
        VkExtent3D {
            width: value.width,
            height: value.height,
            depth: value.depth
        }
    }
}