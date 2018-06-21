extern crate gcc;

use std::*;

fn main() {
    let mut build = gcc::Build::new();
    
    match env::var("VULKAN_SDK") {
        Ok(path) => {
            build.include(format!("{}/include", path));
            println!("cargo:rustc-link-search={}/lib", path);
        },
        Err(_) => ()
    };
    
    build.file("src/c/vkw.c");
    build.compile("libvkw.a");
    
    println!("cargo:rustc-link-lib=vulkan");
}