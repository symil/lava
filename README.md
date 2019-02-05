# Lava

Wrapper to manipulate the Vulkan API in Rust more conveniently than with bindings:

- removes the need to specify the structure type when sending structures to Vulkan
- takes care of making double Vulkan calls when necessary (e.g when retrieving a list of Vulkan objects)
- returns objects retrieved by Vulkan in a `Result` instead of writing them to a user-provided pointer
- allows to manipulate references, slices and options instead of pointers (in particular, allows to provide slice instead of length + pointer)
- exposes the API in an object-oriented way (e.g `instance.enumerate_physical_devices()` instead of `enumerate_physical_devices(&instance)`)
- removes the extension suffix from function and data-structure names (modules are used instead)
- exposes bit flags as structures instead of integers
- provides a default value for all structures, allowing to "auto-complete" structure with optional fields using `..Default::default()`
- manages the calls to `vkGetInstanceProcAddr` to manipulate functions that are not exposed statically
- provides a generic `create_surface` method to create surfaces

Lava is entirely generated from the C header files of Vulkan.

### Restrictions

It comes with the following restrictions (that should be lifted in the future):

- no way to provide allocator callbacks
- no way to set the `pNext` field of structures (always set to `NULL`)

## Usage

Add this dependency to your `Cargo.toml` file:
```
[dependencies]
lava = "0.3.0"
```

## Examples

This code creates a Vulkan instance, adds a debug report callback and displays the name of each GPU of the machine:

```rust
extern crate lava;
use lava::*;

fn main() {
    let instance = vk_create_instance(&VkInstanceCreateInfo {
        flags: VkInstanceCreateFlags::none(),
        application_info: Some(&VkApplicationInfo {
            application_name: Some("lava-example"),
            application_version: 1,
            engine_name: None,
            engine_version: 1,
            api_version: VkVersion(1, 0, 0),
        }),
        enabled_layer_names: &[VK_LAYER_LUNARG_STANDARD_VALIDATION_NAME],
        enabled_extension_names: &[VK_EXT_DEBUG_REPORT_EXTENSION_NAME]
    }).expect("Failed to create instance");

    let debug_report_callback = instance.create_debug_report_callback(&VkDebugReportCallbackCreateInfo {
        flags: VkDebugReportFlags!(warning, error),
        callback: |data: VkDebugReportCallbackMessageData| {
            println!("{}", data.message);
        }
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

## Additional usage information

### Module partitionning

Data-structures are separated in multiple modules, according to their extension (KHR, EXT, etc). Data-structures that have no extension are in the `lava::vk` module.

Constants (e.g validation layer names) are located in the `lava::constants` module.

Lava re-exports all the members of `lava::vk`, `lava::constants`, `lava::ext` and `lava::khr` (`use lava::*` makes all data-structures contained in these modules available without needing to prefix them). This choice was made because Vulkan is a very verbose API, so prefixing everything with a module becomes very tedious very quickly. To avoid conflicts with external data-structures, data-structures are prefixed with `Vk` instead.

### Bit flags

Bit flags are represented as structures instead of integers. Moreover all bit flags structures have static `none()` and `all()` functions. The typical way of creating a bit flags structure is as following:

```rust
// Creates a structure with the `vertex` and `fragment` flag enabled, and all the others disabled
VkShaderStageFlags {
    vertex: true,
    fragment: true,
    ..VkShaderStageFlags::none()
}
```

Since it can be tedious to write, all bit flags structures have a macro shortcut (don't forget `#[macro_use]`!):

```rust
// Same effect as previous snippet
VkShaderStageFlags!(vertex, fragment)
```

### Results

When possible, functions return a `Result<T, (VkResult, T)>`. The return value is `Ok(T)` if the `VkResult` returned by the Vulkan function is 0.
Otherwise it's `Err((VkResult, T))`. The first element of the tuple is the error code returned by the Vulkan function. The second element is, in the specific case where the `VkResult` is not 0 but is not an error either (e.g when calling `swapchain.acquire_next_image()`), the value produced by the function. Otherwise it's a zeroed value that will most likely crash when being used.

## Manual build

The content of the `src/vulkan/` folder is generated from the `vulkan_core.h` and `vk.xml` files of the
[Vulkan documentation repository](https://github.com/KhronosGroup/Vulkan-Docs).
This repository is up to date with the `master` branch.

If you wish to generate the wrapper for a specific version, you can do (requires Node.js):

- `npm install`
- `node generate.js --tag <version>`

Where `<version>` is a branch or tag name of the Vulkan-Docs repository (for example "v1.1.80").
The script will download the corresponding files in the `download/` folder and generate the
new source files in `src/vulkan/`.

## License

[MIT](https://opensource.org/licenses/MIT)
