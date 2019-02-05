// Generated by `scripts/generate.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vulkan::vk::*;

/// Wrapper for [VkExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExtensionProperties.html).
#[derive(Debug, Clone)]
pub struct VkExtensionProperties {
    pub extension_name: String,
    pub spec_version: u32,
}

#[doc(hidden)]
#[repr(C)]
pub struct RawVkExtensionProperties {
    pub extension_name: [c_char; 256],
    pub spec_version: u32,
}

impl VkRawType<VkExtensionProperties> for RawVkExtensionProperties {
    fn vk_to_wrapped(src: &RawVkExtensionProperties) -> VkExtensionProperties {
        VkExtensionProperties {
            extension_name: new_string(&src.extension_name[0] as *const c_char),
            spec_version: src.spec_version,
        }
    }
}

impl VkSetup for VkExtensionProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkExtensionProperties {
    fn vk_free(&mut self) {
        
    }
}