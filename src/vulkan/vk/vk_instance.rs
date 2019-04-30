// Generated by `scripts/generate.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vulkan::*;
use vulkan::vk::*;

#[doc(hidden)]
pub type RawVkInstance = u64;

/// Wrapper for [VkInstance](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkInstance.html).
#[derive(Debug, Clone, Copy)]
pub struct VkInstance {
    _handle: RawVkInstance,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkInstance> for RawVkInstance {
    fn vk_to_wrapped(src: &RawVkInstance) -> VkInstance {
        VkInstance {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkInstance> for VkInstance {
    fn vk_to_raw(src: &VkInstance, dst: &mut RawVkInstance) {
        *dst = src._handle
    }
}

impl Default for VkInstance {
    fn default() -> VkInstance {
        VkInstance {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkInstance {
    fn eq(&self, other: &VkInstance) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkInstance {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkInstance {
    pub fn create_surface<F : Fn(u64, *const c_void, *mut u64) -> i32>(&self, create_fn: F) -> LavaResult<khr::VkSurface> {
        unsafe {
            let raw_surface = &mut mem::zeroed() as *mut khr::RawVkSurface;
            let vk_result = create_fn(self._handle, ptr::null(), raw_surface);
            let mut surface = new_vk_value(raw_surface);
            VkSetup::vk_setup(&mut surface, self._fn_table);
            if vk_result != 0 { return Err((RawVkResult::vk_to_wrapped(&vk_result), surface)) }
            Ok(surface)
        }
    }
}

impl VkInstance {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Indicates if the Vulkan internal handle for this object is 0.
    pub fn is_null(&self) -> bool {
        self._handle == 0
    }
    
    /// Creates an object with a null Vulkan internal handle.
    ///
    /// Calling a method with a null handle will most likely result in a crash.
    pub fn null() -> Self {
        Self {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
    
    /// Wrapper for [vkDestroyInstance](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyInstance.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyInstance)(self._handle, ptr::null());
            if !self._fn_table.is_null() { Box::from_raw(self._fn_table); }
        }
    }
    
    /// Wrapper for [vkEnumeratePhysicalDevices](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkEnumeratePhysicalDevices.html).
    pub fn enumerate_physical_devices(&self) -> LavaResult<Vec<VkPhysicalDevice>> {
        unsafe {
            let mut vk_result = 0;
            let mut raw_physical_devices : *mut RawVkPhysicalDevice = ptr::null_mut();
            let raw_physical_device_count = &mut mem::zeroed() as *mut u32;
            vk_result = ((&*self._fn_table).vkEnumeratePhysicalDevices)(self._handle, raw_physical_device_count, raw_physical_devices);
            raw_physical_devices = calloc(*raw_physical_device_count as usize, mem::size_of::<RawVkPhysicalDevice>()) as *mut RawVkPhysicalDevice;
            
            vk_result = ((&*self._fn_table).vkEnumeratePhysicalDevices)(self._handle, raw_physical_device_count, raw_physical_devices);
            
            let mut physical_devices = new_vk_array(*raw_physical_device_count, raw_physical_devices);
            if vk_result == 0 {
                for elt in &mut physical_devices { VkSetup::vk_setup(elt, self._fn_table); }
            }
            free(raw_physical_devices as *mut u8);
            if vk_result == 0 { Ok(physical_devices) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), physical_devices)) }
        }
    }
    
    /// Wrapper for [vkEnumeratePhysicalDeviceGroups](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html).
    pub fn enumerate_physical_device_groups(&self) -> LavaResult<Vec<VkPhysicalDeviceGroupProperties>> {
        unsafe {
            let mut vk_result = 0;
            let mut raw_physical_device_group_properties : *mut RawVkPhysicalDeviceGroupProperties = ptr::null_mut();
            let raw_physical_device_group_count = &mut mem::zeroed() as *mut u32;
            vk_result = ((&*self._fn_table).vkEnumeratePhysicalDeviceGroups)(self._handle, raw_physical_device_group_count, raw_physical_device_group_properties);
            raw_physical_device_group_properties = calloc(*raw_physical_device_group_count as usize, mem::size_of::<RawVkPhysicalDeviceGroupProperties>()) as *mut RawVkPhysicalDeviceGroupProperties;
            
            vk_result = ((&*self._fn_table).vkEnumeratePhysicalDeviceGroups)(self._handle, raw_physical_device_group_count, raw_physical_device_group_properties);
            
            let mut physical_device_group_properties = new_vk_array(*raw_physical_device_group_count, raw_physical_device_group_properties);
            if vk_result == 0 {
                for elt in &mut physical_device_group_properties { VkSetup::vk_setup(elt, self._fn_table); }
            }
            free(raw_physical_device_group_properties as *mut u8);
            if vk_result == 0 { Ok(physical_device_group_properties) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), physical_device_group_properties)) }
        }
    }
    
    /// Wrapper for [vkCreateDisplayPlaneSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html).
    pub fn create_display_plane_surface(&self, create_info: khr::VkDisplaySurfaceCreateInfo) -> LavaResult<khr::VkSurface> {
        unsafe {
            let raw_create_info = new_ptr_vk_value(&create_info);
            let mut vk_result = 0;
            let raw_surface = &mut mem::zeroed() as *mut khr::RawVkSurface;
            
            vk_result = ((&*self._fn_table).vkCreateDisplayPlaneSurfaceKHR)(self._handle, raw_create_info, ptr::null(), raw_surface);
            
            let mut surface = new_vk_value(raw_surface);
            if vk_result == 0 {
                let fn_table = self._fn_table;
                VkSetup::vk_setup(&mut surface, fn_table);
            }
            free_vk_ptr(raw_create_info);
            if vk_result == 0 { Ok(surface) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), surface)) }
        }
    }
    
    /// Wrapper for [vkCreateDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDebugReportCallbackEXT.html).
    pub fn create_debug_report_callback(&self, create_info: ext::VkDebugReportCallbackCreateInfo) -> LavaResult<ext::VkDebugReportCallback> {
        unsafe {
            let raw_create_info = new_ptr_vk_value(&create_info);
            let mut vk_result = 0;
            let raw_callback = &mut mem::zeroed() as *mut ext::RawVkDebugReportCallback;
            
            vk_result = ((&*self._fn_table).vkCreateDebugReportCallbackEXT)(self._handle, raw_create_info, ptr::null(), raw_callback);
            
            let mut callback = new_vk_value(raw_callback);
            if vk_result == 0 {
                let fn_table = self._fn_table;
                VkSetup::vk_setup(&mut callback, fn_table);
            }
            free_vk_ptr(raw_create_info);
            if vk_result == 0 { Ok(callback) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), callback)) }
        }
    }
    
    /// Wrapper for [vkDebugReportMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDebugReportMessageEXT.html).
    pub fn debug_report_message(&self, flags: ext::VkDebugReportFlags, object_type: ext::VkDebugReportObjectType, object: usize, location: usize, message_code: isize, layer_prefix: &str, message: &str) {
        unsafe {
            let raw_flags = vk_to_raw_value(&flags);
            let raw_object_type = vk_to_raw_value(&object_type);
            let raw_object = vk_to_raw_value(&object);
            let raw_location = location;
            let raw_message_code = vk_to_raw_value(&message_code);
            let raw_layer_prefix = new_ptr_string(layer_prefix);
            let raw_message = new_ptr_string(message);
            ((&*self._fn_table).vkDebugReportMessageEXT)(self._handle, raw_flags, raw_object_type, raw_object, raw_location, raw_message_code, raw_layer_prefix, raw_message);
            free_ptr(raw_layer_prefix);
            free_ptr(raw_message);
        }
    }
    
    /// Wrapper for [vkCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html).
    pub fn create_debug_utils_messenger(&self, create_info: ext::VkDebugUtilsMessengerCreateInfo) -> LavaResult<ext::VkDebugUtilsMessenger> {
        unsafe {
            let raw_create_info = new_ptr_vk_value(&create_info);
            let mut vk_result = 0;
            let raw_messenger = &mut mem::zeroed() as *mut ext::RawVkDebugUtilsMessenger;
            
            vk_result = ((&*self._fn_table).vkCreateDebugUtilsMessengerEXT)(self._handle, raw_create_info, ptr::null(), raw_messenger);
            
            let mut messenger = new_vk_value(raw_messenger);
            if vk_result == 0 {
                let fn_table = self._fn_table;
                VkSetup::vk_setup(&mut messenger, fn_table);
            }
            free_vk_ptr(raw_create_info);
            if vk_result == 0 { Ok(messenger) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), messenger)) }
        }
    }
    
    /// Wrapper for [vkSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html).
    pub fn submit_debug_utils_message(&self, message_severity: ext::VkDebugUtilsMessageSeverityFlags, message_types: ext::VkDebugUtilsMessageTypeFlags, callback_data: ext::VkDebugUtilsMessengerCallbackData) {
        unsafe {
            let raw_message_severity = vk_to_raw_value(&message_severity);
            let raw_message_types = vk_to_raw_value(&message_types);
            let raw_callback_data = new_ptr_vk_value(&callback_data);
            ((&*self._fn_table).vkSubmitDebugUtilsMessageEXT)(self._handle, raw_message_severity, raw_message_types, raw_callback_data);
            free_vk_ptr(raw_callback_data);
        }
    }
    
    /// Wrapper for [vkCreateHeadlessSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateHeadlessSurfaceEXT.html).
    pub fn create_headless_surface(&self, create_info: ext::VkHeadlessSurfaceCreateInfo) -> LavaResult<khr::VkSurface> {
        unsafe {
            let raw_create_info = new_ptr_vk_value(&create_info);
            let mut vk_result = 0;
            let raw_surface = &mut mem::zeroed() as *mut khr::RawVkSurface;
            
            vk_result = ((&*self._fn_table).vkCreateHeadlessSurfaceEXT)(self._handle, raw_create_info, ptr::null(), raw_surface);
            
            let mut surface = new_vk_value(raw_surface);
            if vk_result == 0 {
                let fn_table = self._fn_table;
                VkSetup::vk_setup(&mut surface, fn_table);
            }
            free_vk_ptr(raw_create_info);
            if vk_result == 0 { Ok(surface) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), surface)) }
        }
    }
}