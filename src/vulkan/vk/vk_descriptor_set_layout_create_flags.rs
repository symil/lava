// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDescriptorSetLayoutCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateFlags.html).
///
/// Use the macro `VkDescriptorSetLayoutCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDescriptorSetLayoutCreateFlags!(update_after_bind_pool, push_descriptor_khr)
/// ```
/// ```
/// VkDescriptorSetLayoutCreateFlags {
///     update_after_bind_pool: true,
///     push_descriptor_khr: true,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkDescriptorSetLayoutCreateFlags {
    pub update_after_bind_pool: bool,
    pub push_descriptor_khr: bool,
}

#[doc(hidden)]
pub type RawVkDescriptorSetLayoutCreateFlags = u32;

impl VkWrappedType<RawVkDescriptorSetLayoutCreateFlags> for VkDescriptorSetLayoutCreateFlags {
    fn vk_to_raw(src: &VkDescriptorSetLayoutCreateFlags, dst: &mut RawVkDescriptorSetLayoutCreateFlags) {
        *dst = 0;
        if src.update_after_bind_pool { *dst |= 0x00000002; }
        if src.push_descriptor_khr { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkDescriptorSetLayoutCreateFlags> for RawVkDescriptorSetLayoutCreateFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorSetLayoutCreateFlags) -> VkDescriptorSetLayoutCreateFlags {
        VkDescriptorSetLayoutCreateFlags {
            update_after_bind_pool: (src & 0x00000002) != 0,
            push_descriptor_khr: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkDescriptorSetLayoutCreateFlags {
    fn default() -> VkDescriptorSetLayoutCreateFlags {
        VkDescriptorSetLayoutCreateFlags {
            update_after_bind_pool: false,
            push_descriptor_khr: false,
        }
    }
}

impl VkDescriptorSetLayoutCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDescriptorSetLayoutCreateFlags {
            update_after_bind_pool: false,
            push_descriptor_khr: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDescriptorSetLayoutCreateFlags {
            update_after_bind_pool: true,
            push_descriptor_khr: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.update_after_bind_pool { 0x00000002 } else { 0 }
        + if self.push_descriptor_khr { 0x00000001 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDescriptorSetLayoutCreateFlags {
            update_after_bind_pool: value & 0x00000002 > 0,
            push_descriptor_khr: value & 0x00000001 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDescriptorSetLayoutCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDescriptorSetLayoutCreateFlags {
            $($x: true,)*
            ..VkDescriptorSetLayoutCreateFlags::none()
        }
    }
}