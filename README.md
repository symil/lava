# Lava

Wrapper to manipulate the Vulkan API in Rust more conveniently than with bindings:

- removes the need to specify the structure type when sending structures to Vulkan
- takes care of making double Vulkan calls when necessary (when retrieving a list of Vulkan objects)
- returns objects retrieved by Vulkan in a `Result` instead of requiring a user-provided pointer
- allows to manipulate references, slices and options instead of pointers (in particular, allows to provide slice instead of pointer + length)
- exposes the API in an object-oriented way (e.g `instance.enumerate_physical_devices()` instead of `enumerate_physical_devices(&instance)`)
- removes the extension suffix from function and data-structure names (modules are used instead)
- exposes bit flags as structures instead of integers
- provides a default value for all structures, allowing to "auto-complete" structure with optional fields using `..Default::default()`
- manages the calls to `vkGetInstanceProcAddr` and `vkGetDeviceProcAddr` to manipulate functions that are not exposed statically
- provides a generic `create_surface` method to create surfaces

Lava is entirely generated from the C header files of Vulkan.

### Current restrictions

- no way to provide allocator callbacks
- no way to set the `pNext` field of structures

## Usage

```
[dependencies]
lava = "0.4"
```

## Examples

This code creates a Vulkan instance, adds a debug report callback and displays the name of each GPU of the machine:

```rust
#[macro_use] extern crate lava;
use lava::*;

fn main() {
    let instance = vk_create_instance(VkInstanceCreateInfo {
        flags: VkInstanceCreateFlags!(),
        application_info: Some(VkApplicationInfo {
            application_name: Some("lava-example"),
            application_version: 1,
            engine_name: None,
            engine_version: 1,
            api_version: VkVersion(1, 0, 0),
        }),
        enabled_layer_names: vec![VK_LAYER_KHRONOS_VALIDATION_NAME],
        enabled_extension_names: vec![VK_EXT_DEBUG_REPORT_EXTENSION_NAME]
    }).expect("Failed to create instance");

    let debug_report_callback = instance.create_debug_report_callback(VkDebugReportCallbackCreateInfo {
        flags: VkDebugReportFlags!(error, warning),
        callback: |data: VkDebugReportCallbackData| {
            println!("{}", data.message);
        }
    }).expect("Failed to create debug callback");

    let physical_devices = instance.enumerate_physical_devices()
        .expect("Failed to retrieve physical devices");

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

Some constants (e.g validation layer names) are located in the `lava::constants` module.

Lava re-exports all the members of `lava::vk`, `lava::constants`, `lava::ext` and `lava::khr` ("`use lava::*`" makes all data-structures contained in these modules available without needing to prefix them).

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

Since it can be tedious to write, all bit flags structures have a macro shortcut:

```rust
// Same effect as previous snippet
VkShaderStageFlags!(vertex, fragment)
```

Additionally, all bit flags structures have the following methods:

```rust
let no_shader_stage = VkShaderStageFlags::none();
let all_shader_stages = VkShaderStageFlags::all();
let shader_stages = VkShaderStageFlags::from_u32(17);
let shader_stages_int = shader_stages.to_u32();
```

### Results

When relevant, functions return a `Result<T, (VkResult, T)>`. The return value is `Ok(T)` if the `VkResult` returned by the Vulkan function is 0.
Otherwise it's `Err((VkResult, T))`. The first element of the tuple is the error code returned by the Vulkan function. The second element is, in the specific case where the `VkResult` is not 0 but is not an error either (e.g when calling `swapchain.acquire_next_image()`), the value produced by the function. Otherwise it's a zeroed value that will most likely crash when used.

### Objects destruction and drop

Users are required to manually destroy their objects themselves, instead of Rust doing it automatically when the object is dropped. There are two reasons for that:

- In the C API some objects must not be destroyed by the user. For example, the `VkImage` objects of a swapchain are automatically destroyed when the swapchain is destroyed, and attempting to destroy them manually will produce an error. But the user is still expected to destroy the `VkImage` objects that they create manually. An automatic destruction mechanism would require some context on where the object comes from, and this is out of scope.
- The order in which objects are dropped has a good chance to not match the oder in which they must be destroyed, especially when structures are dropped. 

## Manual build

The content of the `src/vulkan/` folder is generated from the `vulkan_core.h` and `vk.xml` files of the
[Vulkan documentation repository](https://github.com/KhronosGroup/Vulkan-Docs).

If you wish to re-generate it manually, you can do (requires Node.js):

- `npm install`
- `node generate.js [ --tag <version> ]`

Where `<version>` is a branch or tag name of the Vulkan-Docs repository (for example `v1.1.80`).
If omitted, it defaults to `master`.
The script will download the corresponding files in the `download/` folder and generate the new source files.

## License

[MIT](https://opensource.org/licenses/MIT)
