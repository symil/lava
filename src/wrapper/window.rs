use std::ops::Drop;
use std::ffi::*;
use std::*;
use glfw::*;

pub struct Window {
    _window: *mut GlfwWindow
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        unsafe {
            glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
            glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

            let c_string = CString::new(title).unwrap();
            let window = glfwCreateWindow(width as i32, height as i32, c_string.as_ptr(), ptr::null_mut(), ptr::null_mut());

            Window {
                _window: window
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfwDestroyWindow(self._window);
        }
    }
}