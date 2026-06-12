use crate::core::entity::{Entity, RenderedVertex};
use crate::core::mutator::timestamp::TimeStamp;
use crate::core::render::RenderContext;
pub use ndarray::Array2;
use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::Path;
use video_rs::encode::Settings;
use video_rs::{Encoder, Time};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo, RenderPassBeginInfo,
    SubpassBeginInfo, SubpassContents, SubpassEndInfo,
};
use vulkano::format::ClearValue;
use vulkano::pipeline::graphics::viewport::Viewport;

pub trait Canvas {
    fn construct(&self);
    fn get_width_and_height(&self) -> (u32, u32);
    fn get_fps(&self) -> u32;

    fn get_entities(&self) -> &Vec<Box<dyn Entity>>;
    //fn get_background(&self, current_frame: &TimeStamp) -> Array3<u8>;
    fn get_background_color(&self, current_frame: &TimeStamp) -> [u8; 4];

    unsafe fn save(&self, end_dir: &str, name: &str, end: TimeStamp) {
        println!("Starting write");

        let FPS: u32 = self.get_fps();
        let duration: Time = Time::from_nth_of_a_second(FPS as usize);
        let mut position = Time::zero();
        let (WIDTH, HEIGHT): (u32, u32) = self.get_width_and_height();
        let render_context = RenderContext::init().expect("failed to init render context");

        let dir: &str = &format!(
            "{}/{}",
            current_dir()
                .expect("couldn't get current directory")
                .display(),
            end_dir
        );
        create_dir_all(dir).expect("Couldn't make directory");
        let path: &str = &format!("{}/{}", dir, name);
        video_rs::init().unwrap();
        let settings = Settings::preset_h264_yuv420p(WIDTH as usize, HEIGHT as usize, true);
        let mut encoder =
            Encoder::new(Path::new(path), settings).expect("failed to create encoder");

        let mut current_frame = TimeStamp::new_with_defaults(None, None, None);
        let mut image = render_context.init_image(WIDTH, HEIGHT);
        let mut out_buffer = render_context.get_default_image_buffer(
            WIDTH,
            HEIGHT,
            self.get_background_color(&current_frame),
        );

        // compile all the shaders necessary for rendering?

        let VIEWPORT = Viewport {
            offset: [0.0, 0.0],
            extent: [WIDTH as f32, HEIGHT as f32],
            depth_range: 0.0..=1.0,
        };

        while current_frame < end {
            let msaa_image = render_context.init_msaa_image(WIDTH, HEIGHT);
            image = render_context.init_image(WIDTH, HEIGHT);

            let mut builder = AutoCommandBufferBuilder::primary(
                render_context.command_buffer_allocator.clone(),
                render_context.queue_family_index,
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![
                            Some(ClearValue::from(
                                self.get_background_color(&current_frame)
                                    .map(|x| x as f32 / 255.0),
                            )),
                            None,
                        ],
                        ..RenderPassBeginInfo::framebuffer(
                            render_context.init_framebuffer(msaa_image.clone(), image.clone()),
                        )
                    },
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                )
                .unwrap();

            current_frame.increment(FPS);
            println!("processing frame {}", current_frame);

            // build out buffer and attachments that can be shared by entities?

            for entity in self
                .get_entities()
                .iter()
                .filter(|entity| current_frame.matches_range(&entity.active_range()))
            {
                // build out renderpass
                let topology = entity.get_topology();
                let pipeline = render_context.assemble_pipeline(
                    entity.get_vertex_shader(&render_context.default_shaders),
                    entity.get_fragment_shader(&render_context.default_shaders),
                    VIEWPORT.clone(),
                    topology,
                );

                builder.bind_pipeline_graphics(pipeline.clone()).unwrap();

                // Let entity bind its own resources (push constants, descriptor sets, etc.)
                entity.bind_resources(&mut builder, &pipeline, &current_frame, FPS);

                // Render based on computation type
                if entity.uses_gpu_computation() {
                    // GPU computes geometry - create dummy vertex buffer with correct size
                    // Vulkan requires vertex buffer size to match vertex count, even though
                    // our shader doesn't read from it (computes from gl_VertexIndex only)
                    let vertex_count = entity.get_gpu_vertex_count(&current_frame, FPS);
                    let dummy_vertices = vec![
                        RenderedVertex {
                            position: [0.0, 0.0],
                            color: [0.0, 0.0, 0.0, 0.0],
                        };
                        vertex_count as usize
                    ];
                    if let Some(vertex_buffer) = render_context.build_vertex_buffer(dummy_vertices)
                    {
                        builder
                            .bind_vertex_buffers(0, vertex_buffer)
                            .unwrap()
                            .draw(vertex_count, 1, 0, 0)
                            .unwrap();
                    } else {
                        continue;
                    }
                } else {
                    // CPU genates vertices (existing path)
                    let vertices = entity.render(&current_frame, FPS, [WIDTH, HEIGHT]);
                    let num_vertices = vertices.len();
                    if let Some(vertex_buffer) = render_context.build_vertex_buffer(vertices) {
                        builder
                            .bind_vertex_buffers(0, vertex_buffer)
                            .unwrap()
                            .draw(num_vertices as u32, 1, 0, 0)
                            .unwrap();
                    } else {
                        continue;
                    }
                }
            }
            builder.end_render_pass(SubpassEndInfo::default()).unwrap();

            // Copy the rendered image to the output buffer
            builder
                .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                    image.clone(),
                    out_buffer.clone(),
                ))
                .unwrap();

            let command_buffer = builder.build().unwrap();
            encoder
                .encode(
                    &render_context.render_frame(
                        command_buffer.clone(),
                        &mut out_buffer,
                        WIDTH as usize,
                        HEIGHT as usize,
                    ),
                    position,
                )
                .expect("failed to encode frame");

            // Update the current position and add the inter-frame duration to it.
            position = position.aligned_with(duration).add();
        }
        encoder.finish().expect("failed to finish encoder");
    }
}
