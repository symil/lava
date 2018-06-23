#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::ffi::*;

mod vk_types;
use vk_types::vk_bool32::RawVkBool32;

#[repr(C)]
struct VkResult {
    vk_result: i64,
    ptr: *mut VkInstance
}

#[repr(C)]
struct VkInstance {
    dummy: u64
}

#[repr(i32)]
#[derive(PartialEq)]
enum Kind {
    Heart = 0,
    Diamond = 4,
    Spade = 7,
    Club = 15
}

impl convert::From<Kind> for bool {
    fn from(value: Kind) -> Self {
        value != Kind::Heart
    }
}

impl convert::From<i32> for Kind {
    fn from(value: i32) -> Self {
        match value {
            0 => Kind::Heart,
            4 => Kind::Diamond,
            7 => Kind::Spade,
            15 => Kind::Club,
            _ => panic!("unable to convert integer {} to enum Kind", value)
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Heart => write!(f, "Heart"),
            Kind::Diamond => write!(f, "Diamond"),
            Kind::Spade => write!(f, "Spade"),
            Kind::Club => write!(f, "Club")
        }
    }
}

extern {
    fn get_kind() -> Kind;
    fn add_one(x: u32) -> u32;
    fn vk_create_instance() -> VkResult;
    fn vk_destroy_instance(instance: *mut VkInstance);
    fn get_first_device_name(instance: *mut VkInstance) -> *mut i8;
}

fn main() {
    unsafe {
        // println!("Result from Rust: {}", result.vk_result)
        
        // if get_kind() as bool {
        //     println!("Yes");
        // } else {
        //     println!("No");
        // }
        // println!("Value: {}", get_kind());
        
        // let result = vk_create_instance();
        // let instance = result.ptr;    
        // let str_ptr = get_first_device_name(instance);
        // let c_name = CString::from_raw(str_ptr);
        // let name = c_name.into_string().unwrap();
        
        // println!("{}", name);
        
        // vk_destroy_instance(instance);
    }
    let a : RawVkBool32 = 32;
    println!("{}", bool::from(a));

    println!("Bye!");
}