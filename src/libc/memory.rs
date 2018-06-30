use std::vec::Vec;
use std::string::String;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::boxed::Box;
use std::*;
use libc::*;

pub unsafe fn copy_as_string(ptr: *const c_char) -> String {
    String::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes().to_vec())
}

pub unsafe fn copy_as_c_string(s: &String) -> *mut c_char {
    let str_len = s.len();
    let new_str = calloc(1, str_len + 1);
    memcpy(new_str, s.as_ptr() as *const void, str_len);

    new_str
}

pub unsafe fn free_c_string(ptr: *mut void) {
    free(ptr);
}

pub unsafe fn copy_as_c_string_array(strings: &Vec<String>) -> *mut *mut c_char {
    let array_len = strings.len();
    let c_array = calloc(array_len + 1, mem::size_of::<usize>()) as *mut *mut c_char;

    for (i, s) in strings.iter().enumerate() {
        let str_copy = copy_as_c_string(s);
        *(c_array.offset(i as isize)) = str_copy;
    }

    c_array
}

pub unsafe fn free_c_string_array(c_strings: *mut *mut c_char) {
    let mut ptr : *mut *mut c_char = c_strings;

    while !(*ptr).is_null() {
        free(*ptr);
        ptr = ptr.offset(1);
    }
}

pub unsafe fn copy_as_c_ptr<T>(v: T) -> *mut T {
    Box::into_raw(Box::from(v))
}

pub unsafe fn free_c_ptr<T>(ptr: *mut T) {
    Box::from_raw(ptr);
}

pub unsafe fn copy_as_c_array<T>(v: &Vec<T>) -> *mut T {
    let byte_len = v.len() * mem::size_of::<usize>();
    let ptr = malloc(byte_len);
    memcpy(ptr, v.as_ptr() as *const void, byte_len);

    ptr as *mut T
}

pub unsafe fn free_c_array<T>(ptr: *mut T) {
    free(ptr as *mut void);
}

pub unsafe fn vec_from_c_ptr<T : Copy>(length: u32, ptr: *const T) -> Vec<T> {
    let length_usize = length as usize;
    let mut result : Vec<T> = Vec::with_capacity(length_usize);
    result.set_len(length_usize);

    for i in 0..length_usize {
        result[i] = *(ptr.offset(i as isize));
    }

    result
}