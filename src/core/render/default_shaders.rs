pub mod simple_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
                #version 460

                layout(location = 0) in vec2 position;
                layout(location = 1) in vec4 color;
                
                layout(location = 0) out vec4 f_color;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                    f_color = color;
                }
            ",
    }
}

pub mod flat_colored_fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
                #version 460
                layout(location = 0) in vec4 f_color;
                layout(location = 0) out vec4 frag_color;

                void main() {
                    frag_color = f_color;
                }
            ",
    }
}
