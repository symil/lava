use os::raw::c_char;
use libc::void;

pub type GlfwMonitor = void;
pub type GlfwWindow = void;

extern {
    pub fn glfwInit();
    pub fn glfwTerminate();
    pub fn glfwCreateWindow(width: i32, height: i32, title: *const c_char, monitor: *mut GlfwMonitor, share: *mut GlfwWindow) -> *mut GlfwWindow;
    pub fn glfwDestroyWindow(window: *mut GlfwWindow);
    pub fn glfwWindowHint(hint: i32, value: i32);
    pub fn glfwWindowShouldClose(window: *mut GlfwWindow) -> i32;
    pub fn glfwPollEvents();
}

pub const GLFW_CLIENT_API : i32 = 0x00022001;
pub const GLFW_NO_API : i32 = 0;
pub const GLFW_RESIZABLE : i32 = 0x00020003;
pub const GLFW_FALSE: i32 = 0;