// Copied from `scripts/static/`

use vk::VkResult;

/// Wraps both the return code and the value produced by a Vulkan function.
/// 
/// In the C API, Vulkan functions return the code that indicates if they were successful or not. The "actuel" return value is
/// written through a user-specified pointer. Lava replaces this mechanism with the convenient `Result`: if the return code is`VK_SUCCESS`
/// the function returns `Ok`, otherwise it returns `Err`.
/// 
/// In some cases a non-`VK_SUCCESS` code does not indicate a failure. This is for example the case of
/// [vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAcquireNextImageKHR.html).
/// To account for these cases, the argument of `Err` is a tuple made of the return code and the produced value. If no value
/// is produced (e.g because the operation was a failure), the second value of the tuple is a zeroed value.
pub type LavaResult<T> = Result<T, (VkResult, T)>;