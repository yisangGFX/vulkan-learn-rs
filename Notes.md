# Environments

- vulkano = "0.33.0" :a safe wrapper around the Vulkan API with some convenience functionality

- winit = "0.28.6" ：an alternative window managment library written in pure Rust

# Vulkan Steps
Here is the steps to create a simple vulkan application:
1. base code(winit, lib, ...)
2. create VulkanLibrary
3. create an instance
4. Surface
5. Physical devices and queue families
6. Logic device and queues

# Others
- A lot of information in Vulkan is passed through structs instead of function parameters and we'll have to fill in one more struct to provide sufficient information for creating an instance.

## externsions
Vulkan是跨平台的，扩展允许硬件制造商、操作系统供应商或其他第三方提供超出Vulkan核心规范之外的功能，在创建instance的时候可以根据dirven确定启用哪些features

## instance
instance provide some useful information to the driver in order to optimize our specific application
## Validation layers
为了减少驱动的开销，Vulkand的设计理念是尽可能少的运行时调试开销，这玩意是开发时帮助开发者进行调试和诊断的，正式的application不会加这个。所以我看的example基本没实现这个。

## Physical devices
 A PhysicalDevice represents a Vulkan-capable device that is available on the system, such as a graphics card, a software implementation, etc.