# Lava: a Rust wrapper for the Vulkan API

Wrapper to manipulate the Vulkan API more conveniently than with bindings:

- removes the need to specify the structure type when sending structures to Vulkan
- takes care of making double Vulkan calls when necessary (e.g when retrieving a list of Vulkan objects)
- returns objects retrieved by Vulkan in a `Result<T, VkResult>` instead of returning them via a pointer
- allows to manipulate references, slices and options instead of pointers
- exposes the API in an object-oriented way (e.g `instance.enumerate_physical_devices()` instead of `enumerate_physical_devices(&instance)`)
- removes the extension suffix from function and data-structure names
- puts all data-structures with the same extension in a dedicated module
- exposes bit flags as structures instead of integers
- provides a default value for all structures, allowing to "auto-complete" structure with optional fields using `..Default::default()`
- manages the calls to `vkGetInstanceProcAddr` to manipulate functions that are not exposed statically
- provides a generic `create_surface` method to create surfaces

Lava works by letting the developer manipulate "wrapped" data structures, which it internally converts to "raw" data-structures
expected by Vulkan (and the other way around when retrieving objects from Vulkan).
It means that there is a small overhead in each API call.

### Restrictions

It comes with the following restrictions (that should be lifted in the future):

- no way to provide allocator callbacks
- no way to set the `pNext` field of structures (always set to `NULL`)
- debug report callbacks only forward the message to the Rust user-provided function (other pieces of information are unavailable)
- no exposed constants for validation layer names or extension names

## Usage

Add this dependency to your `Cargo.toml` file:
```
[dependencies]
lava = "0.1.0"
```

## Examples

This code adds a debug report callback and displays the name of each GPU of the machine:

```rust
extern crate lava;
use lava::*;

fn main() {
    let instance = Vk::create_instance(&VkInstanceCreateInfo {
        flags: VkInstanceCreateFlags::none(),
        application_info: Some(&VkApplicationInfo {
            application_name: Some("lava-example"),
            application_version: 1,
            engine_name: None,
            engine_version: 1,
            api_version: VkVersion(1, 0, 0),
        }),
        enabled_layer_names: &["VK_LAYER_LUNARG_standard_validation"],
        enabled_extension_names: &["VK_EXT_debug_report"]
    }).expect("Failed to create instance");

    let debug_report_callback = instance.create_debug_report_callback(&VkDebugReportCallbackCreateInfo {
        flags: VkDebugReportFlags {
            warning: true,
            error: true,
            ..VkDebugReportFlags::none()
        },
        callback: |msg : String| println!("{}", msg)
    }).expect("Faield to create debug callback");

    let physical_devices = instance.enumerate_physical_devices().expect("Failed to retrieve physical devices");

    for physical_device in &physical_devices {
        let properties = physical_device.get_properties();

        println!("{}", properties.device_name);
    }

    debug_report_callback.destroy();
    instance.destroy();
}
```

This snippet shows how to create a surface from a GLFW window:

```rust
// We assume that `window` is a pointer to a GLFWwindow, as described here:
// http://www.glfw.org/docs/latest/group__vulkan.html#ga1a24536bec3f80b08ead18e28e6ae965

let surface = instance.create_surface(
    |handle, allocator, surface| unsafe { glfwCreateWindowSurface(handle, window, allocator, surface) }
).expect("Failed to create surface from glfw window");
```

## Manual build

The content of the `src/vk/` folder is generated from the `vulkan_core.h` and `vk.xml` files of the
[Vulkan documentation repository](https://github.com/KhronosGroup/Vulkan-Docs).
This repository is up to date with the `master` branch.

If you wish to generate the wrapper for a specific version, you can do (requires Node.js):

- `npm install`
- `node generate.js --tag <version>`

Where `<version>` is a branch or tag name of the Vulkan-Docs repository (for example "v1.1.80").
The script will download the corresponding files in the `download/` folder and generate the
new source files in `src/vk/`.

## Warning

This is very much a work in progress. Most of the API has not been tested.

## License

[MIT](https://opensource.org/licenses/MIT)
