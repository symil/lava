use std::vec::Vec;
use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::*;
use utils::*;

pub fn copy_as_c_string(s: &String) -> *mut void {
    unsafe {
        let str_len = s.len();
        let new_str = calloc(1, str_len + 1);
        memcpy(new_str, s.as_ptr() as *const void, str_len);

        new_str
    }
}

pub fn free_c_string(ptr: *mut void) {
    unsafe {
        free(ptr);
    }
}

pub fn copy_as_c_string_array(strings: &Vec<String>) -> *mut *mut void {
    unsafe {
        let array_len = strings.len();
        let c_array = calloc(array_len + 1, mem::size_of::<usize>()) as *mut *mut void;

        for (i, s) in strings.iter().enumerate() {
            let str_copy = copy_as_c_string(s);
            *(c_array.offset(i as isize)) = str_copy;
        }

        c_array
    }
}

pub fn free_c_string_array(c_strings: *mut *mut void) {
    unsafe {
        let mut ptr : *mut *mut void = c_strings;

        while !(*ptr).is_null() {
            free(*ptr);
            ptr = ptr.offset(1);
        }
    }
}