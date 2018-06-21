#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ptr;

#[repr(C)]
struct VkResult {
    vk_resut: i64,
    ptr: *mut VkInstance
}

#[repr(C)]
struct VkInstance {
    dummy: i32
}

extern {
    fn add_one(x: u32) -> u32;
    fn vk_create_instance() -> VkResult;
    fn vk_destroy_instance(instance: *mut VkInstance);
}

fn main() {
    unsafe {
        let result = vk_create_instance();
        let instance = result.ptr;    
        vk_destroy_instance(instance);
    }
    
    println!("Bye!");
}