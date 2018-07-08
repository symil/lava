#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::os::raw::c_char;
use std::ffi::*;
use std::fmt;

mod vk;
use vk::*;

mod libc;
use libc::*;

mod glfw;
use glfw::*;

// fn display_properties(value: &VkPhysicalDeviceProperties) {
//     println!("API version   : {:#?}", value.api_version);
//     println!("Driver version: {:#?}", value.driver_version);
//     println!("Vendor ID     : {:#?}", value.vendor_id);
//     println!("Device ID     : {:#?}", value.device_id);
//     println!("Device type   : {:#?}", value.device_type);
//     println!("Device name   : {:#?}", value.device_name);
// }

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;

const LAYER_STANDARD_VALIDATION : &str = "VK_LAYER_LUNARG_standard_validation";

const EXT_DEBUG_REPORT : &str = "VK_EXT_debug_report";
const EXT_SWAPCHAIN : &str = "VK_KHR_swapchain";

type VkDebugReportCallbackEXT = usize;
type DebugCallback = unsafe extern "C" fn(flags: u32, obj_type: i32, obj: u64, location: usize, code: i32, layer_prefix: *const c_char, msg: *const c_char, user_data: *mut c_void) -> RawVkBool32;
type VkCreateDebugReportCallback = unsafe extern "C" fn(instance: RawVkInstance, create_info: *const VkDebugReportCallbackCreateInfoEXT, allocator: *const c_void, p_callback: *mut VkDebugReportCallbackEXT) -> RawVkResult;

unsafe extern "C" fn test_debug_callback(flags: u32, obj_type: i32, obj: u64, location: usize, code: i32, layer_prefix: *const c_char, msg: *const c_char, user_data: *mut c_void) -> RawVkBool32 {
    println!("{:?}", CStr::from_ptr(msg));

    VK_FALSE
}

#[repr(C)]
struct VkDebugReportCallbackCreateInfoEXT {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    callback: DebugCallback,
    user_data: *mut c_void
}

unsafe fn create_debug_report_callback_ext(instance: RawVkInstance, create_info: *const VkDebugReportCallbackCreateInfoEXT) -> Result<VkDebugReportCallbackEXT, RawVkResult> {
    let ext_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap().into_raw();
    let func_ptr = vkGetInstanceProcAddr(instance, ext_name);

    if !func_ptr.is_null() {
        let func : VkCreateDebugReportCallback = mem::transmute(func_ptr);
        let mut handle : VkDebugReportCallbackEXT = 0;
        let handle_ptr = &mut handle as *mut VkDebugReportCallbackEXT;

        func(instance, create_info, ptr::null(), handle_ptr);

        println!("Exists!");
        Ok(handle)
    } else {
        println!("Does not exist.");
        Err(VkFrom::vk_from(&VkResult::ErrorExtensionNotPresent))
    }
}

extern {
    fn vkGetInstanceProcAddr(instance: RawVkInstance, name: *const c_char) -> *const c_void;
}

fn main() {
    let mut required_extensions : Vec<String> = vec![];
    let validation_layers = vec![String::from(LAYER_STANDARD_VALIDATION)];

    let glfw = GlfwInstance::new();
    let window = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vulkan");

    required_extensions.append(&mut glfw.get_required_vulkan_extensions().unwrap());
    required_extensions.push(String::from(EXT_DEBUG_REPORT));

    let instance = VkInstance::new(&VkInstanceCreateInfo {
        flags: VkInstanceCreateFlags { },
        application_info: VkApplicationInfo {
            application_name: String::from("foo"),
            application_version: [0, 1, 0],
            engine_name: String::from("engine"),
            engine_version: [0, 1, 0],
            api_version: [1, 0, 0]
        },
        enabled_layer_names: validation_layers.clone(),
        enabled_extension_names: required_extensions
    }).expect("Failed to create VkInstance");

    unsafe {
        let create_info = VkDebugReportCallbackCreateInfoEXT {
            // s_type: VkFrom::vk_from(&VkStructureType::DebugReportCallbackCreateInfoExt),
            s_type: 1000011000,
            p_next: ptr::null(),
            flags: 2 | 8,
            callback: test_debug_callback,
            user_data: ptr::null_mut()
        };

        create_debug_report_callback_ext(instance.handle(), &create_info as *const VkDebugReportCallbackCreateInfoEXT).expect("Failed to add validation layer");
    }

    let surface = instance.create_surface_from_glfw(&window).expect("Failed to create VkSurface");

    let physical_devices = instance.get_physical_devices().expect("Failed to retrieve physical devices");
    let physical_device = &physical_devices[0];
    let features = physical_device.get_features();
    let properties = physical_device.get_properties();
    let queue_families = physical_device.get_queue_families();
    let queue_family = &queue_families[0];
    let device = physical_device.create_logical_device(&VkDeviceCreateInfo {
        flags: VkDeviceCreateFlags::none(),
        queue_create_infos: vec![
            VkDeviceQueueCreateInfo {
                flags: VkDeviceQueueCreateFlags::none(),
                queue_family_index: 0,
                queue_priorities: vec![1.0]
            }
        ],
        enabled_layer_names: vec![],
        enabled_extension_names: vec![String::from(EXT_SWAPCHAIN)],
        enabled_features: VkPhysicalDeviceFeatures::none()
    }).expect("Failed to initialize VkDevice");

    let queue = device.get_queue(0, 0);
    let buffer = device.create_buffer(&VkBufferCreateInfo {
        size: 128,
        flags: VkFlags::none(),
        usage: VkFlags::all(),
        sharing_mode: VkSharingMode::Exclusive,
        queue_family_indices: Vec::new()
    }).expect("Failed to create VkBuffer");

    let is_surface_supported = physical_device.does_support_surface(0, &surface).unwrap();
    let instance_supported_extensions : Vec<String> = instance.get_supported_extensions().unwrap().into_iter().map(|ext| ext.extension_name).collect();
    let device_supported_extensions : Vec<String> = physical_device.get_supported_extensions().expect("Failed to retrieve device extensions").into_iter().map(|ext| ext.extension_name).collect();
    let capabilities = physical_device.get_surface_capabilities(&surface).expect("Failed to retrieve surface capabilities");
    let surface_present_modes = physical_device.get_surface_present_modes(&surface).expect("Failed to retrieve present modes");
    let layers = instance.get_layer_properties().expect("Failed to retrieve layer properties");
    let current_transform = capabilities.current_transform.clone();

    // println!("{:#?}", capabilities);
    // println!("{:#?}", current_transform);

    let swapchain = device.create_swapchain(&VkSwapchainCreateInfoKHR {
        flags: VkSwapchainCreateFlagsKHR::none(),
        surface: &surface,
        min_image_count: 2,
        image_format: VkFormat::B8G8R8A8Unorm,
        image_color_space: VkColorSpaceKHR::SrgbNonlinearKhr,
        image_extent: VkExtent2D {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT
        },
        image_array_layers: 1,
        image_usage: VkImageUsageFlags {
            color_attachment: true,
            ..VkImageUsageFlags::none()
        },
        image_sharing_mode: VkSharingMode::Exclusive,
        queue_family_indices: vec![],
        pre_transform: capabilities.current_transform.clone(),
        composite_alpha: VkCompositeAlphaFlagsKHR {
            opaque_khr: true,
            ..VkCompositeAlphaFlagsKHR::none()
        },
        present_mode: VkPresentModeKHR::FifoKhr,
        clipped: false,
        old_swapchain: None
    }).expect("Failed to create VkSwapchain");

    let images = swapchain.get_images().expect("Failed to retrieve images from swapchain");

    let image_view = device.create_image_view(&VkImageViewCreateInfo {
        flags: VkFlags::none(),
        image: &images[0],
        view_type: VkImageViewType::_2d,
        format: VkFormat::B8G8R8A8Unorm,
        components: VkComponentMapping {
            r: VkComponentSwizzle::Identity,
            g: VkComponentSwizzle::Identity,
            b: VkComponentSwizzle::Identity,
            a: VkComponentSwizzle::Identity
        },
        subresource_range: VkImageSubresourceRange {
            aspect_mask: VkImageAspectFlags {
                color: true,
                ..VkFlags::none()
            },
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1
        }
    }).expect("Failed to create VkImageView");

    window.start_loop();

    println!("Bye!");
}