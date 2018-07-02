use std::*;
use std::option::Option;
use std::vec::Vec;
use std::convert::*;
use vk::VkResult;

pub type VkHandle = usize;
pub const VK_NULL_HANDLE : VkHandle = 0;

pub trait VkFrom<T> {
    fn from(&T) -> Self;
}

pub fn vk_make_version(version: &[u32; 3]) -> u32 {
    (((version[0]) << 22) | ((version[1]) << 12) | (version[2]))
}

pub unsafe fn vk_call_retrieve_list<T, U, F>(vk_func: F) -> Result<Vec<U>, VkResult>
    where
        U: VkFrom<T>,
        F: Fn(*mut u32, *mut T) -> VkResult
{
    let mut count : u32 = 0;
    let mut vector : Vec<T> = Vec::new();
    let result = vk_func(&mut count as *mut u32, ptr::null_mut());

    match result {
        VkResult::Success => {
            vector.reserve(count as usize);
            vector.set_len(count as usize);
            vk_func(&mut count as *mut u32, vector.as_mut_ptr());

            Ok(vector.iter().map(|raw| VkFrom::from(raw)).collect())
        },
        _ => Err(result)
    }
}

pub unsafe fn vk_call_retrieve_list_unchecked<T, U, F>(vk_func: F) -> Vec<U>
    where
        U: VkFrom<T>,
        F: Fn(*mut u32, *mut T)
{
    vk_call_retrieve_list(|count, ptr| { vk_func(count, ptr); VkResult::Success }).unwrap()
}

pub unsafe fn vk_call_retrieve_single<T, U, F, C>(vk_func: F, callback: C) -> Result<U, VkResult>
    where
        U: VkFrom<T>,
        F: Fn(*mut T) -> VkResult,
        C: Fn(&mut U)
{
    let mut raw : T = mem::uninitialized();
    let result = vk_func(&mut raw as *mut T);

    match result {
        VkResult::Success => {
            let mut value = VkFrom::from(&raw);
            callback(&mut value);
            Ok(value)
        },
        _ => Err(result)
    }
}

pub unsafe fn vk_call_retrieve_single_unchecked<T, U, F, C>(vk_func: F, callback: C) -> U
    where
        U: VkFrom<T>,
        F: Fn(*mut T),
        C: Fn(&mut U)
{
    vk_call_retrieve_single(|ptr| { vk_func(ptr); VkResult::Success }, callback).unwrap()
}

pub fn to_vk_bool(value: bool) -> u32 {
    if value { 1 } else { 0 }
}