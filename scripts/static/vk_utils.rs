use std::*;
use std::os::raw::c_char;
use std::vec::Vec;
use std::convert::*;
use libc::c_void;
use vk::*;

pub type RawVkHandle = u64;

pub const VK_NULL_HANDLE : RawVkHandle = 0;
pub const VK_SUCCESS : RawVkResult = 0;
pub const VK_FALSE : RawVkBool32 = 0;
pub const VK_TRUE : RawVkBool32 = 1;

pub trait VkFrom<T> {
    fn vk_from(&T) -> Self;
}

#[repr(C)]
struct VkPtrBox<T> {
    ptr: *mut T
}

impl<T> VkPtrBox<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(value))
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }
}

impl<T> Drop for VkPtrBox<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}

struct VkRefBox<T> {
    value: T
}

impl<T> VkRefBox<T> {
    fn new(value: T) -> Self {
        Self {
            value: value
        }
    }
}

impl<T> Deref for VkRefBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

struct VkNullType;
static VK_NULL_VALUE : VkNullType = VkNullType {};

pub trait VkWrappedType<R> {
    fn vk_from_raw(&R) -> Self;
    fn vk_default() -> Self;

    fn vk_null() -> &Self {
        unsafe {
            ((&VK_NULL_VALUE as *const VkNullType) as *const Self).as_ref().unwrap()
        }
    }

    fn vk_is_null(value: &Self) -> bool {
        value as *const Self == (&VK_NULL_VALUE as *const NullType) as *const Self
    }

    fn vk_ref_from_raw_ptr(value_ptr: *const R) -> VkRefBox<Self> {
        VkRefBox::new(Self::vk_from_raw( unsafe { &*value_ptr }))
    }
}

pub trait VkRawType<W> {
    fn vk_from_wrapped(&W) -> Self;

    fn vk_ptr_from_wrapped_ref(value_ptr: &W) -> VkPtrBox<Self> {
        VkPtrBox::new(Self::vk_from_wrapped(value))
    }
}

pub trait VkFlags {
    fn none() -> Self;
    fn all() -> Self;
}

pub const VK_STRING_CAPACITY : usize = 256;
pub struct VkRawString {
    chars: [c_char; VK_STRING_CAPACITY]
}

impl VkRawType<str> for VkRawString {
    vk_from_wrapped(value: &str) -> Self {
        
    }
}

pub fn vk_make_version(version: &[u32; 3]) -> u32 {
    (((version[0]) << 22) | ((version[1]) << 12) | (version[2]))
}

pub fn vk_from_version(value: u32) -> [u32; 3] {
    [
        value >> 22,
        (value >> 12) % (1 << 10),
        value % (1 << 12)
    ]
}