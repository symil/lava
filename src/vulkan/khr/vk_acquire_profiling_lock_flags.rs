// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkAcquireProfilingLockFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAcquireProfilingLockFlagsKHR.html).
///
/// Use the macro `VkAcquireProfilingLockFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkAcquireProfilingLockFlags!()
/// ```
/// ```
/// VkAcquireProfilingLockFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkAcquireProfilingLockFlags {
    
}

#[doc(hidden)]
pub type RawVkAcquireProfilingLockFlags = u32;

impl VkWrappedType<RawVkAcquireProfilingLockFlags> for VkAcquireProfilingLockFlags {
    fn vk_to_raw(src: &VkAcquireProfilingLockFlags, dst: &mut RawVkAcquireProfilingLockFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkAcquireProfilingLockFlags> for RawVkAcquireProfilingLockFlags {
    fn vk_to_wrapped(src: &RawVkAcquireProfilingLockFlags) -> VkAcquireProfilingLockFlags {
        VkAcquireProfilingLockFlags {
            
        }
    }
}

impl Default for VkAcquireProfilingLockFlags {
    fn default() -> VkAcquireProfilingLockFlags {
        VkAcquireProfilingLockFlags {
            
        }
    }
}

impl VkAcquireProfilingLockFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkAcquireProfilingLockFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkAcquireProfilingLockFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkAcquireProfilingLockFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkAcquireProfilingLockFlags {
    ( $( $x:ident ),* ) => {
        VkAcquireProfilingLockFlags {
            $($x: true,)*
            ..VkAcquireProfilingLockFlags::none()
        }
    }
}