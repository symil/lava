use std::ffi::CString;
use std::ffi::CStr;
use std::cmp;
use std::mem;
use std::os::raw::c_char;
use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use vulkan::vk::*;

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

pub fn vk_to_wrapped_array<W, R : VkRawType<W>>(array: &[R], dst: &mut[W]) {
    let len = cmp::min(array.len(), dst.len());

    for i in 0..len {
        dst[i] = R::vk_to_wrapped(&array[i])
    }
}

pub fn fill_vk_array<T : Default>(dst: &mut[T]) {
    for i in 0..dst.len() {
        dst[i] = T::default();
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

pub fn new_vk_array<R : VkRawType<W>, W>(length: u32, ptr: *const R) -> Vec<W> {
    unsafe {
        let len = length as usize;
        let mut vector : Vec<W> = Vec::with_capacity(len);

        for i in 0..len {
            vector.push(R::vk_to_wrapped(&*ptr.add(i)));
        }

        vector
    }
}

pub fn new_vk_array_checked<R : VkRawType<W>, W>(length: u32, ptr: *const R) -> Option<Vec<W>> {
    if ptr.is_null() {
        None
    } else {
        Some(new_vk_array(length, ptr))
    }
}

pub fn new_vk_value<R : VkRawType<W>, W>(ptr: *const R) -> W {
    unsafe {
        R::vk_to_wrapped(&*ptr)
    }
}

pub fn new_vk_value_checked<R : VkRawType<W>, W>(ptr: *const R) -> Option<W> {
    unsafe {
        if ptr.is_null() {
            None
        } else {
            Some(R::vk_to_wrapped(&*ptr))
        }
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
        // TODO: remove condition?
        if ptr.is_null() {
            String::from("")
        } else {
            String::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes().to_vec())
        }
    }
}

pub fn new_string_checked(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        Some(new_string(ptr))
    } else {
        None
    }
}

pub fn new_string_vec(length: u32, ptr: *const *const c_char) -> Vec<String> {
    let len = length as usize;
    let mut result : Vec<String> = Vec::with_capacity(len);

    for i in 0..len {
        result.push(new_string(unsafe { *ptr.add(i) }));
    }

    result
}

pub fn vec_from_ptr<T>(length: usize, ptr: *const T) -> Vec<T> {
    unsafe {
        Vec::from_raw_parts(mem::transmute(ptr), length, length)
    }
}

pub fn vec_from_ptr_checked<T>(length: usize, ptr: *const T) -> Option<Vec<T>> {
    if ptr.is_null() {
        None
    } else {
        Some(vec_from_ptr(length, ptr))
    }
}

pub unsafe fn get_vk_instance_function_pointer(instance: RawVkInstance, name: &str) -> *const c_void {
    let c_string = CString::new(name).unwrap();

    vkGetInstanceProcAddr(instance, c_string.as_c_str().as_ptr())
}

pub unsafe fn get_vk_device_function_pointer(device: RawVkDevice, name: &str) -> *const c_void {
    let c_string = CString::new(name).unwrap();

    vkGetDeviceProcAddr(device, c_string.as_c_str().as_ptr())
}

extern {
    fn vkGetInstanceProcAddr(instance: RawVkInstance, name: *const c_char) -> *const c_void;
    fn vkGetDeviceProcAddr(device: RawVkDevice, name: *const c_char) -> *const c_void;
}