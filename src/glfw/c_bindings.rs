use os::raw::c_char;
use libc::void;
use vk::*;

pub type RawGlfwMonitor = void;
pub type RawGlfwWindow = void;

extern {
    pub fn glfwInit();
    pub fn glfwTerminate();
    pub fn glfwCreateWindow(width: i32, height: i32, title: *const c_char, monitor: *mut RawGlfwMonitor, share: *mut RawGlfwWindow) -> *mut RawGlfwWindow;
    pub fn glfwDestroyWindow(window: *mut RawGlfwWindow);
    pub fn glfwWindowHint(hint: i32, value: i32);
    pub fn glfwWindowShouldClose(window: *mut RawGlfwWindow) -> i32;
    pub fn glfwPollEvents();
    pub fn glfwCreateWindowSurface(vk_instance: RawVkInstance, window: *mut RawGlfwWindow, allocator: *const VkAllocator, surface: *mut RawVkSurface) -> VkResult;
    pub fn glfwGetRequiredInstanceExtensions(count: *mut u32) -> *const *const c_char;
}

pub const GLFW_CLIENT_API : i32 = 0x00022001;
pub const GLFW_NO_API : i32 = 0;
pub const GLFW_RESIZABLE : i32 = 0x00020003;
pub const GLFW_FALSE: i32 = 0;