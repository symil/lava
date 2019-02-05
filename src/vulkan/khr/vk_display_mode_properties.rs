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
use vulkan::khr::{VkDisplayMode,RawVkDisplayMode};
use vulkan::khr::{VkDisplayModeParameters,RawVkDisplayModeParameters};

/// Wrapper for [VkDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayModePropertiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkDisplayModeProperties {
    pub display_mode: VkDisplayMode,
    pub parameters: VkDisplayModeParameters,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayModeProperties {
    pub display_mode: RawVkDisplayMode,
    pub parameters: RawVkDisplayModeParameters,
}

impl VkRawType<VkDisplayModeProperties> for RawVkDisplayModeProperties {
    fn vk_to_wrapped(src: &RawVkDisplayModeProperties) -> VkDisplayModeProperties {
        VkDisplayModeProperties {
            display_mode: RawVkDisplayMode::vk_to_wrapped(&src.display_mode),
            parameters: RawVkDisplayModeParameters::vk_to_wrapped(&src.parameters),
        }
    }
}

impl VkSetup for VkDisplayModeProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.display_mode, fn_table);
        VkSetup::vk_setup(&mut self.parameters, fn_table);
    }
}

impl VkFree for RawVkDisplayModeProperties {
    fn vk_free(&mut self) {
        RawVkDisplayModeParameters::vk_free(&mut self.parameters);
    }
}