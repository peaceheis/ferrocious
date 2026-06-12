# ferrocious

## Introduction

ferrocious is a general-purpose, code-based animation engine. It is meant to be error-free, expressive, easy, extensible and efficient.
It was born out of the desire to create imaginative geometric animations with code,
and the lack of tools that empower it. Here are the priorities of its design.

0. **Pure**
From the ground up, ferrocious was meant to give you full control over every single pixel of every single frame - to that end, rendering code should be as stateless and as functional as possible, a crucial part of the design philosophy of ferrocious.
GPU and CPU-based shaders get passed time and pixel data, and every effort is made to clear the way for this powerful style of animation. The following rules provide structure for the scaffolding around this, especially regarding the STL.
1. **Error-Free**
ferrocious should guarantee safety, in line with Rust's philosophy. Additionally, if you interact with ferrocious' API and STL, any error you will encounter should be
encountered during compile time or early in runtime. By the time rendering starts, predictable errors from reasonable validation should have already been caught. Crashes after long render times are incredibly inconvenient and ultimately unacceptable. Frontload errors.
2. **Expressive**
    You should be able to say what you want with code -- to that end, STL should be well-written enough that solutions can be easily adapted, and it should be *possible* to write it this well, without 
complication. The STL serves as the primary benchmark of core library clarity -- it should be implementable purely with the core library; no special hooks.
3. **Easy**
    You should be able to create any animation if you're clever enough. But you shouldn't have to be. The standard library should have the solutions to as many graphical problems as possible.
4. **Extensible**
    For the most creative, one's imagination may stretch farther than ferrocious' capabilities. When that's the case, ferrocious should be a platform, not an obstacle.
As an abstraction, ferrocious should be an open fence that you can reach through (an idea taken from PySimpleGUI), and not a brick wall you must break through. 
5. **Efficient**
Lastly, ferrocious must be performant, in both speed and memory. Internally, this may require some convolution, but this priority must rarely, if ever, supersede the others. The use of Rust and Vulkan provides 
both complexity and efficiency enough. Performance is built into the design, but it should never feel like the API surface sacrifices clarity for performance. In reasonable conflicts between speed and memory, speed should be chosen.