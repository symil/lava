use std::string::*;
use std::vec::*;
use std::ops::Drop;
use utils::vk_convert::new_string;
use glfw::*;

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

    pub fn create_window(&self, width: usize, height: usize, title: &str) -> GlfwWindow {
        GlfwWindow::new(width, height, title)
    }

    pub fn get_required_vulkan_extensions(&self) -> Option<Vec<String>> {
        unsafe {
            let mut count : u32 = 0;
            let names = glfwGetRequiredInstanceExtensions(&mut count as *mut u32);

            if names.is_null() {
                None
            } else {
                let mut string_vec : Vec<String> = Vec::new();

                for i in 0..count as usize {
                    string_vec.push(new_string(*names.add(i)))
                }

                Some(string_vec)
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