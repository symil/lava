use std::ops::Drop;
use std::ffi::*;
use std::*;
use glfw::*;
use vk::*;

pub struct GlfwWindow {
    _window: *mut RawGlfwWindow
}

impl GlfwWindow {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        unsafe {
            glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
            glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

            let c_string = CString::new(title).unwrap();
            let window = glfwCreateWindow(width as i32, height as i32, c_string.as_ptr(), ptr::null_mut(), ptr::null_mut());

            GlfwWindow {
                _window: window
            }
        }
    }

    pub fn start_loop(&self) {
        unsafe {
            while glfwWindowShouldClose(self._window) == 0 {
                glfwPollEvents();
            }
        }
    }

    pub fn handle(&self) -> *mut RawGlfwWindow {
        self._window
    }
}

impl Drop for GlfwWindow {
    fn drop(&mut self) {
        unsafe {
            glfwDestroyWindow(self._window);
        }
    }
}