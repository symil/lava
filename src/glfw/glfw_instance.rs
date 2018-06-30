use std::ops::Drop;
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

    pub fn create_window(&self, width: u32, height: u32, title: &str) -> GlfwWindow {
        GlfwWindow::new(width, height, title)
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