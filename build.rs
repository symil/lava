use std::env;

fn main() {
     match env::var("VULKAN_SDK") {
        Ok(path) => {
            println!("cargo:rustc-link-search={}/lib", path);
        },
        Err(_) => ()
    };

    let libs_to_link = vec!["glfw3", "rt", "m", "dl", "Xrandr", "Xinerama", "Xxf86vm", "Xext", "Xcursor", "Xrender", "Xfixes", "X11", "pthread", "xcb", "Xau", "Xdmcp", "vulkan"];

    for lib in libs_to_link.iter() {
        println!("cargo:rustc-link-lib={}", lib);
    }
}