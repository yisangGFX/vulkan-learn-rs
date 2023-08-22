因为网上大部分都是英文资料，所以第一次弄双语笔记（英文是抄的或者简单的表达，中文是自己想的比较复杂懒得转换成英语了，我的英语啥时候能信手拈来啊）
# Environments

- vulkano = "0.33.0" :a safe wrapper around the Vulkan API with some convenience functionality

- winit = "0.28.6" ：an alternative window managment library written in pure Rust

# Vulkan
Here is the steps to create a simple vulkan application:
1. base code
2. create an instance
   instance provide some useful information to the driver in order to optimize our specific application
3. Validation layers
4. Physical devices and queue families

# Others
- A lot of information in Vulkan is passed through structs instead of function parameters and we'll have to fill in one more struct to provide sufficient information for creating an instance.

## externsions
Vulkan是跨平台的，扩展允许硬件制造商、操作系统供应商或其他第三方提供超出Vulkan核心规范之外的功能，在创建instance的时候可以根据dirven确定启用哪些features
## Validation layers
为了减少驱动的开销，Vulkand的设计理念是尽可能少的运行时调试开销，这玩意是开发时帮助开发者进行调试和诊断的，正式的application不会加这个。所以我看的example基本没实现这个。
