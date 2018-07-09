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

pub unsafe fn copy_as_string_vec(count: u32, strings: *const *const c_char) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();

    for i in 0..count as usize {
        let ptr = *(strings.offset(i as isize));
        result.push(copy_as_string(ptr));
    }

    result
}

pub unsafe fn copy_as_c_string(s: &String) -> *mut c_char {
    let str_len = s.len();
    let new_str = calloc(1, str_len + 1);
    memcpy(new_str, s.as_ptr() as *const c_void, str_len);

    new_str
}

pub unsafe fn free_c_string(ptr: *mut c_void) {
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
    let byte_len = v.len() * mem::size_of::<T>();
    let ptr = malloc(byte_len);
    memcpy(ptr, v.as_ptr() as *const c_void, byte_len);

    ptr as *mut T
}

pub unsafe fn free_c_array<T>(ptr: *mut T) {
    free(ptr as *mut c_void);
}

pub unsafe fn vec_from_c_ptr<T : Copy>(length: u32, ptr: *const T) -> Vec<T> {
    let len = length as usize;
    let t_size = mem::size_of::<T>();
    let copy_ptr = malloc(len * t_size) as *mut T;

    for i in 0..len {
        let src_ptr = ptr.offset(i as isize);
        let dst_ptr = copy_ptr.offset(i as isize);
        memcpy(dst_ptr as *mut c_void, src_ptr as *mut c_void, t_size);
    }

    Vec::from_raw_parts(copy_ptr, len, len)
}