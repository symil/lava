#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::*;
use std::os::raw::c_char;
use std::ffi::*;
use std::fmt;

mod vk;
use vk::*;
use vk::ext::*;
use vk::khr::*;

mod utils;
// use utils::*;

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

const WINDOW_WIDTH : usize = 800;
const WINDOW_HEIGHT : usize = 600;

const LAYER_STANDARD_VALIDATION : &str = "VK_LAYER_LUNARG_standard_validation";

const EXT_DEBUG_REPORT : &str = "VK_EXT_debug_report";
const EXT_SWAPCHAIN : &str = "VK_KHR_swapchain";

fn simple_debug_callback(msg: String) {
    println!("{}", msg);
}

fn main() {
    let glfw = GlfwInstance::new();
    let glfw_required_extensions = glfw.get_required_vulkan_extensions().unwrap();
    let window = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vulkan");

    let mut required_extensions = vec![EXT_DEBUG_REPORT];

    for ext_name in &glfw_required_extensions {
        required_extensions.push(&ext_name)
    }

    let supported_extensions = Vk::enumerate_instance_extension_properties(None).expect("Failed to retrieve supported extensions");

    let instance = Vk::create_instance(&VkInstanceCreateInfo {
        flags: VkInstanceCreateFlags { },
        application_info: Some(&VkApplicationInfo {
            application_name: Some("foo"),
            application_version: 1,
            engine_name: Some("engine"),
            engine_version: 1,
            api_version: VkVersion(1, 0, 0),
        }),
        enabled_layer_names: &[LAYER_STANDARD_VALIDATION],
        enabled_extension_names: &required_extensions
    }).expect("Failed to create VkInstance");

    let debug_report_callback = instance.create_debug_report_callback(&VkDebugReportCallbackCreateInfo {
        flags: VkDebugReportFlags {
            warning: true,
            error: true,
            ..VkDebugReportFlags::none()
        },
        callback: simple_debug_callback
    }).expect("Faield to create debug callback");


    let surface = window.create_vk_surface(&instance).expect("Failed to create VkSurface");

    let physical_devices = instance.enumerate_physical_devices().expect("Failed to retrieve physical devices");

    let physical_device = &physical_devices[0];
    let features = physical_device.get_features();
    let properties = physical_device.get_properties();

    let queue_families = physical_device.get_queue_family_properties();
    let queue_family = &queue_families[0];
    let device = physical_device.create_device(&VkDeviceCreateInfo {
        flags: VkDeviceCreateFlags::none(),
        queue_create_infos: &[
            VkDeviceQueueCreateInfo {
                flags: VkDeviceQueueCreateFlags::none(),
                queue_family_index: 0,
                queue_priorities: &[1.0]
            }
        ],
        enabled_layer_names: &[],
        enabled_extension_names: &[EXT_SWAPCHAIN],
        enabled_features: None
    }).expect("Failed to initialize VkDevice");

    let queue = device.get_queue(0, 0);
    let buffer = device.create_buffer(&VkBufferCreateInfo {
        flags: VkBufferCreateFlags::none(),
        size: 128,
        usage: VkBufferUsageFlags {
            transfer_src: true,
            ..VkBufferUsageFlags::none()
        },
        sharing_mode: VkSharingMode::Exclusive,
        queue_family_indices: &[]
    }).expect("Failed to create VkBuffer");

    let is_surface_supported = physical_device.get_surface_support(0, &surface).unwrap();
    let device_supported_extensions = physical_device.enumerate_device_extension_properties(None).expect("Failed to retrieve device extensions");
    let capabilities = physical_device.get_surface_capabilities(&surface).expect("Failed to retrieve surface capabilities");
    let formats = physical_device.get_surface_formats(&surface).expect("Failed to retrieve surface formats");
    let surface_present_modes = physical_device.get_surface_present_modes(&surface).expect("Failed to retrieve present modes");
    let current_transform = capabilities.current_transform.clone();

    let swapchain = device.create_swapchain(&VkSwapchainCreateInfo {
        flags: VkSwapchainCreateFlags::none(),
        surface: &surface,
        min_image_count: 2,
        image_format: VkFormat::B8G8R8A8Unorm,
        image_color_space: VkColorSpace::SrgbNonlinear,
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
        queue_family_indices: &[],
        pre_transform: capabilities.current_transform.clone(),
        composite_alpha: VkCompositeAlphaFlags {
            opaque: true,
            ..VkCompositeAlphaFlags::none()
        },
        present_mode: VkPresentMode::Fifo,
        clipped: false,
        old_swapchain: None
    }).expect("Failed to create VkSwapchain");

    let images = swapchain.get_images().expect("Failed to retrieve images from swapchain");

    let image_view = device.create_image_view(&VkImageViewCreateInfo {
        flags: VkImageViewCreateFlags::none(),
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
                ..VkImageAspectFlags::none()
            },
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1
        }
    }).expect("Failed to create VkImageView");

    let vertex_spirv_bytes = include_bytes!("spirv/shader.vert.spv");
    let fragment_spirv_bytes = include_bytes!("spirv/shader.frag.spv");

    let vertex_shader_module = device.create_shader_module(&VkShaderModuleCreateInfo {
        flags: VkShaderModuleCreateFlags::none(),
        code: vertex_spirv_bytes
    }).expect("Failed to create vertex shader module");

    let fragment_shader_module = device.create_shader_module(&VkShaderModuleCreateInfo {
        flags: VkShaderModuleCreateFlags::none(),
        code: fragment_spirv_bytes
    }).expect("Failed to create fragment shader module");

    let pipeline_layout = device.create_pipeline_layout(&VkPipelineLayoutCreateInfo::default()).expect("Failed to create VkPipelineLayout");

    window.start_loop();

    pipeline_layout.destroy();
    fragment_shader_module.destroy();
    vertex_shader_module.destroy();
    image_view.destroy();
    swapchain.destroy();
    buffer.destroy();
    device.destroy();
    surface.destroy();
    debug_report_callback.destroy();
    instance.destroy();

    println!("Bye!");
}