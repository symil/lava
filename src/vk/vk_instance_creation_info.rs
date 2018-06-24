use vk::*;
use std::default;
use std::vec::Vec;
use std::string::String;

#[derive(Debug)]
pub struct VkInstanceCreateInfo<'a> {
    pub application_info: VkApplicationInfo<'a>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<String>
}

impl<'a> default::Default for VkInstanceCreateInfo<'a> {
    fn default() -> VkInstanceCreateInfo<'a> {
        VkInstanceCreateInfo {
            application_info: VkApplicationInfo::default(),
            enabled_layers: Vec::new(),
            enabled_extensions: Vec::new()
        }
    }
}