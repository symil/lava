use std::string::String;
use std::ffi::CString;
use std::ffi::CStr;
use std::vec::Vec;
use std::cmp;
use std::mem;
use std::os::raw::c_char;
use utils::vk_type::*;
use utils::vk_ptr::*;
use vk::vk_instance::*;
use vk::vk_result::*;

pub fn vk_to_raw_value<W : VkWrappedType<R>, R>(value: &W) -> R {
    unsafe {
        let mut dst = mem::uninitialized();
        W::vk_to_raw(value, &mut dst);
        dst
    }
}

pub fn vk_to_raw_array<W : VkWrappedType<R>, R>(array: &[W], dst: &mut[R]) {
    let len = cmp::min(array.len(), dst.len());

    for i in 0..len {
        W::vk_to_raw(&array[i], &mut dst[i])
    }
}

pub fn fill_vk_array<T : VkDefault>(dst: &mut[T]) {
    for i in 0..dst.len() {
        dst[i] = T::vk_default();
    }
}

pub fn to_array<T : Copy>(src: &[T], dst: &mut[T]) {
    let len = cmp::min(src.len(), dst.len());

    for i in 0..len {
        dst[i] = src[i];
    }
}

pub fn string_to_byte_array(string: &str, dst: &mut[c_char]) {
    let bytes = string.as_bytes();
    let len = cmp::min(bytes.len(), dst.len() - 1);

    for i in 0..len {
        dst[i] = bytes[i] as c_char;
    }

    dst[len] = 0;
}

pub fn new_vk_array<R : VkRawType<W>, W>(length: u32, ptr: *const R) -> Vec<W>
{
    unsafe {
        let len = length as usize;
        let mut vector : Vec<W> = Vec::with_capacity(len);

        for i in 0..len {
            vector.push(R::vk_to_wrapped(&*ptr.add(i)));
        }

        vector
    }
}

pub fn new_array<T : Copy>(length: u32, ptr: *const T) -> Vec<T> {
    unsafe {
        let len = length as usize;
        let mut vector : Vec<T> = Vec::with_capacity(len);

        for i in 0..len {
            vector.push(*ptr.add(i));
        }

        vector
    }
}

pub fn new_string(ptr: *const c_char) -> String {
    unsafe {
        String::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes().to_vec())
    }
}

pub fn new_string_checked(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        Some(new_string(ptr))
    } else {
        None
    }
}

pub unsafe fn get_vk_instance_function_pointer(instance: RawVkInstance, name: &str) -> *mut c_void {
    let c_string = CString::new(name).unwrap();

    vkGetInstanceProcAddr(instance, c_string.as_c_str().as_ptr())
}

extern {
    fn vkGetInstanceProcAddr(instance: RawVkInstance, name: *const c_char) -> *mut c_void;
}