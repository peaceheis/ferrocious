# Ferrocious

## Introduction

Ferrocious is a general-purpose, code-based animation engine. It is meant to be error-free, expressive, easy, and efficient.
It was born out of the desire to create imaginative geometric animations with code,
and the failure of tools to be able to do so. Here are the priorities of its design.

## Design principles

1. **Error-Free**
Ferrocious should guarantee safety, in line with Rust's philosophy. Additionally, if you interact with Ferrocious' API and STL, any error you will encounter should be
encountered during compile time or early in runtime. Crashes after long render times are incredibly inconvenient and ultimately unacceptable. Frontload errors.
2. **Expressive**
    You should be able to say what you want with code. There should be nothing, truly nothing, that you couldn't implement if you're not clever enough, and just with the core library.
From the ground up, Ferrocious was meant to give you full control over every single pixel of every single frame - to that end, rendering code should be as time-agnostic as possible.
In fact, Manim (a large philosophical inspiration for this project (though not very much so in design)) and its time-dependence was a large motivator for this project.
3. **Easy**
    You should be able to create any animation if you're clever enough. But you shouldn't have to be. The standard library should have the solutions to as many graphical problems as possible.
4. **Extensible**
    For the most creative, one's imagination may stretch farther than Ferrocious' capabilities. When that's the case, Ferrocious should be a platform, not an obstacle.
As an abstraction, Ferrocious should be an open fence that you can reach through (an idea taken from PySimpleGUI), and not a brick wall you must break through.
5. **Efficient**
    Lastly, Ferrocious must be performant, in both speed and memory. Internally, this may require some convolution, but this priority must rarely, if ever, supersede the others. The use of Rust and Vulkan provides 
both complexity and efficiency enough. Performance is built into the design, but it should never feel like the API surface sacrifices clarity for performance. In reasonable conflicts between speed and memory, speed should be chosen.