# Environments

- vulkano = "0.33.0" :a safe wrapper around the Vulkan API with some convenience functionality

- winit = "0.28.6" ：an alternative window managment library written in pure Rust

# Vulkan
Here is the steps to create a simple vulkan application:
1. base code
2. create an instance
   instance provide some useful information to the driver in order to optimize our specific application
3. Validation layers

# Others
- A lot of information in Vulkan is passed through structs instead of function parameters and we'll have to fill in one more struct to provide sufficient information for creating an instance.

