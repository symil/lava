use std::string::*;
use std::vec::*;
use std::ops::Drop;
use glfw::*;
use libc::*;

static mut NB_INSTANCES : u32 = 0;

pub struct GlfwInstance;

impl GlfwInstance {
    pub fn new() -> Self {
        unsafe {
            if NB_INSTANCES == 0 {
                glfwInit();
            }

            NB_INSTANCES += 1;

            GlfwInstance
        }
    }

    pub fn create_window(&self, width: u32, height: u32, title: &str) -> GlfwWindow {
        GlfwWindow::new(width, height, title)
    }

    pub fn get_required_vulkan_extensions(&self) -> Option<Vec<String>> {
        unsafe {
            let mut count : u32 = 0;
            let names = glfwGetRequiredInstanceExtensions(&mut count as *mut u32);

            if names.is_null() {
                None
            } else {
                let c_string_vec = vec_from_c_ptr(count, names);
                Some(c_string_vec.into_iter().map(|c_string| copy_as_string(c_string)).collect())
            }
        }
    }
}

impl Drop for GlfwInstance {
    fn drop(&mut self) {
        unsafe {
            NB_INSTANCES -= 1;

            if NB_INSTANCES == 0 {
                glfwTerminate();
            }
        }
    }
}