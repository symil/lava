use std::mem;
use std::ptr;
use std::os::raw::c_char;
use utils::vk_null::vk_is_null;
use utils::vk_type::VkRawType;
use utils::vk_type::VkWrappedType;

#[allow(non_camel_case_types)]
pub type c_void = u8;

extern {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
}

const SIZE_OF_PTR : usize = mem::size_of::<*const u8>();

#[repr(C)]
pub struct VkPtr<R> {
    ptr: *mut R
}

impl<R> VkPtr<R> {
    pub fn new_value(value: R) -> Self
        where R : Copy
    {
        unsafe {
            let ptr = malloc(mem::size_of::<R>()) as *mut R;
            *ptr = value;

            Self {
                ptr: ptr
            }
        }
    }

    pub fn new_array(array: &[R]) -> Self
        where R : Copy
    {
        unsafe {
            let ptr = malloc(mem::size_of::<R>() * array.len()) as *mut R;

            for i in 0..array.len() {
                *ptr.add(i) = array[i];
            }

            Self {
                ptr: ptr
            }
        }
    }

    pub fn new_vk_value<W>(value: &W) -> Self
        where W : VkWrappedType<R>
    {
        unsafe {
            if vk_is_null(value) {
                return Self {
                    ptr: ptr::null_mut() as *mut R
                }
            }

            let ptr = malloc(mem::size_of::<R>()) as *mut R;
            let dst = ptr.as_mut().unwrap();
            W::vk_to_raw(value, dst);

            Self {
                ptr: ptr
            }
        }
    }

    pub fn new_vk_value_checked<W>(value: Option<&W>) -> Self
        where W : VkWrappedType<R>
    {
        match value {
            Some(v) => Self::new_vk_value(v),
            None => Self { ptr: ptr::null_mut() }
        }
    }

    pub fn new_vk_value_array<W : VkWrappedType<R>>(array: &[W]) -> Self {
        unsafe {
            let byte_len = array.len() * mem::size_of::<W>();
            let ptr = malloc(byte_len) as *mut R;

            for i in 0..array.len() {
                let dst = ptr.add(i).as_mut().unwrap();
                W::vk_to_raw(&array[i], dst);
            }

            Self {
                ptr: ptr
            }
        }
    }

    pub fn new_null() -> Self {
        Self {
            ptr: ptr::null_mut() as *mut R
        }
    }

    pub fn as_ptr(&self) -> *const R {
        self.ptr as *const R
    }
}

impl VkPtr<c_char> {
    pub fn new_string(string: &str) -> Self {
        unsafe {
            let bytes = string.as_bytes();
            let len = bytes.len();
            let ptr = malloc(len + 1) as *mut c_char;

            for i in 0..len {
                *ptr.add(i) = bytes[i] as c_char;
            }

            *ptr.add(len) = 0;

            Self {
                ptr: ptr
            }
        }
    }

    pub fn new_string_checked(string: Option<&str>) -> Self {
        match string {
            Some(value) => Self::new_string(value),
            None => Self { ptr: ptr::null_mut() }
        }
    }
}

impl VkPtr<*mut c_char> {
    pub fn new_string_array(array: &[&str]) -> Self {
        unsafe {
            let nb_strings = array.len();
            let mut total_strings_len : usize = 0;

            for i in 0..array.len() {
                total_strings_len += array[i].len();
            }

            let byte_len = total_strings_len + (SIZE_OF_PTR + 1) * nb_strings;
            let ptr = malloc(byte_len) as *mut c_char;
            let addr_ptr = ptr as *mut *mut c_char;
            let mut write_start_addr = ptr.add(SIZE_OF_PTR * nb_strings);

            for i in 0..nb_strings {
                let bytes = array[i].as_bytes();
                let len = bytes.len();

                *addr_ptr.add(i) = write_start_addr;

                for j in 0..len {
                    *write_start_addr.add(j) = bytes[j] as c_char;
                }

                *write_start_addr.add(len) = 0;
                write_start_addr = write_start_addr.add(len + 1);
            }

            Self {
                ptr: addr_ptr
            }
        }
    }
}

impl<T> Drop for VkPtr<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut c_void);
        }
    }
}