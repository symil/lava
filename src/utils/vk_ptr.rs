use std::mem;
use std::ptr;
use std::os::raw::c_char;
use utils::c_bindings::*;
use utils::vk_traits::*;

const SIZE_OF_PTR : usize = mem::size_of::<*const u8>();

pub fn free_ptr<T>(ptr: *mut T) {
    unsafe {
        free(ptr as *mut c_void);
    }
}

pub fn free_vk_ptr<T : VkFree>(ptr: *mut T) {
    unsafe {
        if !ptr.is_null() {
            (&mut *ptr).vk_free();
            free_ptr(ptr);
        }

    }
}

pub fn free_vk_ptr_array<T : VkFree>(size: u32, ptr: *mut T) {
    unsafe {
        if !ptr.is_null() {
            for i in 0..size as usize {
                (&mut *ptr.add(i)).vk_free();
            }

            free_ptr(ptr);
        }
    }
}

pub fn new_ptr_value<R : Copy>(value: R) -> *mut R {
    unsafe {
        let ptr = malloc(mem::size_of::<R>()) as *mut R;
        *ptr = value;

        ptr
    }
}

pub fn new_ptr_array<R : Copy>(array: &[R]) -> *mut R {
    unsafe {
        if array.len() == 0 {
            return ptr::null_mut()
        }

        let ptr = malloc(mem::size_of::<R>() * array.len()) as *mut R;

        for i in 0..array.len() {
            *ptr.add(i) = array[i];
        }

        ptr
    }
}

pub fn new_ptr_vk_value<R, W : VkWrappedType<R>>(value: &W) -> *mut R {
    unsafe {
        let ptr = malloc(mem::size_of::<R>()) as *mut R;
        let dst = ptr.as_mut().unwrap();
        W::vk_to_raw(value, dst);

        ptr
    }
}

pub fn new_ptr_vk_value_checked<R, W : VkWrappedType<R>>(value: Option<&W>) -> *mut R {
    match value {
        Some(v) => new_ptr_vk_value(v),
        None => ptr::null_mut()
    }
}

pub fn new_ptr_vk_array<R, W : VkWrappedType<R>>(array: &[W]) -> *mut R {
    unsafe {
        if array.len() == 0 {
            return ptr::null_mut()
        }

        let byte_len = array.len() * mem::size_of::<R>();
        let ptr = malloc(byte_len) as *mut R;

        for i in 0..array.len() {
            let dst = ptr.add(i).as_mut().unwrap();
            W::vk_to_raw(&array[i], dst);
        }

        ptr
    }
}

pub fn new_ptr_string(string: &str) -> *mut c_char {
    unsafe {
        let bytes = string.as_bytes();
        let len = bytes.len();
        let ptr = malloc(len + 1) as *mut c_char;

        for i in 0..len {
            *ptr.add(i) = bytes[i] as c_char;
        }

        *ptr.add(len) = 0;

        ptr
    }
}

pub fn new_ptr_string_checked(string: Option<&str>) -> *mut c_char {
    match string {
        Some(value) => new_ptr_string(value),
        None => ptr::null_mut()
    }
}

pub fn new_ptr_string_array(array: &[&str]) -> *mut *mut c_char {
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

        addr_ptr
    }
}

pub fn new_ptr_vk_array_array<R, W : VkWrappedType<R>>(array: &[&W]) -> *mut *mut R {
    unsafe {
        let nb_elements = array.len();
        if nb_elements == 0 {
            return ptr::null_mut()
        }

        let byte_len = nb_elements * (SIZE_OF_PTR + mem::size_of::<R>());
        let ptr = malloc(byte_len);
        let ptr_addr = ptr as *mut *mut R;
        let ptr_content = ptr.add(SIZE_OF_PTR * nb_elements) as *mut R;

        for i in 0..nb_elements {
            let elt_ptr = ptr_content.add(i);
            W::vk_to_raw(array[i], &mut *elt_ptr);
            *ptr_addr.add(i) = elt_ptr;
        }

        ptr_addr
    }
}