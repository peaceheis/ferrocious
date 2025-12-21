use std::any::Any;
use ndarray::Array3;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use video_rs::Frame;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, PrimaryAutoCommandBuffer,
};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::{ClearColorValue, Format};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::Vertex as VulkanoVertex;
use vulkano::pipeline::graphics::vertex_input::VertexDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::sync::GpuFuture;
use vulkano::{sync, VulkanLibrary};
use crate::core::entity::RenderedVertex;
use crate::core::render::default_shaders::{flat_colored_fragment_shader, simple_vertex_shader};
use crate::core::render::rgba::RGBA;

pub mod default_shaders;
pub mod pipeline_layouts;
pub mod rgba;
mod GraphicsPassContext;

/// Key for caching graphics pipelines
/// Pipelines are uniquely identified by their shaders and viewport dimensions
#[derive(Hash, Eq, PartialEq, Clone)]
struct PipelineKey {
    // Use Arc::as_ptr() for comparison - same shader = same pointer
    vertex_shader_ptr: usize,
    fragment_shader_ptr: usize,
    viewport_width: u32,
    viewport_height: u32,
}

impl PipelineKey {
    fn new(vs: &Arc<ShaderModule>, fs: &Arc<ShaderModule>, viewport: &Viewport) -> Self {
        Self {
            vertex_shader_ptr: Arc::as_ptr(vs) as usize,
            fragment_shader_ptr: Arc::as_ptr(fs) as usize,
            viewport_width: viewport.extent[0] as u32,
            viewport_height: viewport.extent[1] as u32,
        }
    }
}

pub struct RenderContext {
    physical_device: Arc<PhysicalDevice>,
    pub logical_device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub queue_family_index: u32,
    memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub render_pass: Arc<RenderPass>,
    pub default_shaders: DefaultShaders,
    // Pipeline cache for performance (uses RefCell for interior mutability)
    pipeline_cache: RefCell<HashMap<PipelineKey, Arc<GraphicsPipeline>>>,
}

impl RenderContext {
    pub fn init() -> Result<RenderContext, Box<dyn std::error::Error>> {
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(library, InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        }).expect("failed to create Vulkan instance");

        // device & queue creation
        let physical_device = Self::init_physical_device(&instance);
        let queue_family_index = Self::init_queue_family_index(&physical_device);
        let (logical_device, mut queues) =
            Self::init_logical_device(&physical_device, queue_family_index);
        let queue = queues.next().unwrap();

        // buffer creation
        let memory_allocator =
            Arc::new(StandardMemoryAllocator::new_default(logical_device.clone()));
        let command_buffer_allocator =
            Arc::new(Self::init_command_buffer_allocator(logical_device.clone()));

        let render_pass = Self::init_render_pass(logical_device.clone());

        let default_shaders = Self::init_default_shaders(logical_device.clone());

        Ok(RenderContext {
            physical_device,
            logical_device,
            queue,
            queue_family_index,
            memory_allocator,
            command_buffer_allocator,
            render_pass,
            default_shaders,
            pipeline_cache: RefCell::new(HashMap::new()),
        })
    }

    fn init_physical_device(instance: &Arc<Instance>) -> Arc<PhysicalDevice> {
        instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available")
    }

    fn init_queue_family_index(physical_device: &Arc<PhysicalDevice>) -> u32 {
        physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family") as u32
    }

    fn init_logical_device(
        physical_device: &Arc<PhysicalDevice>,
        queue_family_index: u32,
    ) -> (Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>) {
        Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device")
    }

    fn init_default_shaders(device: Arc<Device>) -> DefaultShaders {
        DefaultShaders {
            SIMPLE_VERTEX_SHADER: simple_vertex_shader::load(device.clone())
                .expect("failed to initialize SIMPLE VERTEX SHADER"),
            FLAT_COLORED_FRAGMENT_SHADER: flat_colored_fragment_shader::load(device.clone())
                .expect("failed to initialize SIMPLE VERTEX SHADER"),
        }
    }

    fn init_command_buffer_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
        StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        )
    }

    pub fn init_image(&self, width: u32, height: u32) -> Arc<Image> {
        Image::new(
            self.memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_UNORM,
                extent: [width, height, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC | ImageUsage::COLOR_ATTACHMENT |ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )
        .unwrap()
    }

    pub fn get_default_image_buffer(&self, width: u32, height: u32, default_color: [u8; 4]) -> Subbuffer<[u8]> {

        Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            (0..width * height * 4).map(|i| default_color[i as usize %4]),
        )
        .expect("failed to create buffer")
    }

    // TODO: image format compatability, more versatile default image?
    pub fn create_clear_image_command_buffer(
        &self,
        image: Arc<Image>,
        color: RGBA,
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let mut builder = AutoCommandBufferBuilder::primary(
            self.command_buffer_allocator.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .clear_color_image(ClearColorImageInfo {
                clear_value: ClearColorValue::Float([
                    color.red,
                    color.green,
                    color.blue,
                    color.alpha,
                ]),
                ..ClearColorImageInfo::image(image.clone())
            })
            .unwrap();

        builder.build().unwrap()
    }

    fn init_render_pass(device: Arc<Device>) -> Arc<vulkano::render_pass::RenderPass> {
        vulkano::single_pass_renderpass!(device,
            attachments: {
                color: {
                    format: Format::R8G8B8A8_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap()
    }

    pub fn init_framebuffer(&self, image: Arc<Image>) -> Arc<Framebuffer> {
        let view = ImageView::new_default(image).unwrap();
        Framebuffer::new(
            self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            },
        )
        .unwrap()
    }

    pub fn build_vertex_buffer(
        &self,
        vertices: Vec<RenderedVertex>,
    ) -> Subbuffer<[RenderedVertex]> {
        // Ensure we have vertices to work with
        if vertices.is_empty() {
            panic!("Cannot create buffer from empty vertex vector");
        }

        // Try with more permissive memory type filter first
        Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices.into_iter(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create vertex buffer: {:?}. This might indicate insufficient memory or invalid device state.", e)
        })
    }

    pub fn assemble_pipeline(
        &self,
        vs: Arc<ShaderModule>,
        fs: Arc<ShaderModule>,
        viewport: Viewport,
    ) -> Arc<GraphicsPipeline> {
        let key = PipelineKey::new(&vs, &fs, &viewport);

        // Check cache first
        {
            let cache = self.pipeline_cache.borrow();
            if let Some(pipeline) = cache.get(&key) {
                return pipeline.clone();
            }
        }

        // Create new pipeline if not cached
        let vs = vs.entry_point("main").unwrap();
        let fs = fs.entry_point("main").unwrap();

        let vertex_input_state = RenderedVertex::per_vertex().definition(&vs).unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            self.logical_device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(self.logical_device.clone())
                .unwrap(),
        )
        .unwrap();

        let subpass = Subpass::from(self.render_pass.clone(), 0).unwrap();

        let pipeline = GraphicsPipeline::new(
            self.logical_device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap();

        // Cache the pipeline for future use
        self.pipeline_cache.borrow_mut().insert(key, pipeline.clone());

        pipeline
    }

    pub fn render_frame(
        &self,
        command_buffer: Arc<PrimaryAutoCommandBuffer>,
        buf: &mut Subbuffer<[u8]>,
        WIDTH: usize,
        HEIGHT: usize,
    ) -> Frame {
        let future = sync::now(self.logical_device.clone())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();
        let buffer_content = buf.read().unwrap();
        
        
        // Convert RGBA to RGB (remove alpha channel)
        let mut rgb_data = Vec::with_capacity(WIDTH * HEIGHT * 3);
        for chunk in buffer_content.chunks_exact(4) {
            rgb_data.extend_from_slice(&chunk[..3]); // Take only R, G, B (skip A)
        }
        
        Array3::from_shape_vec((HEIGHT, WIDTH, 3), rgb_data).unwrap()

    }
}

pub struct DefaultShaders {
    pub SIMPLE_VERTEX_SHADER: Arc<ShaderModule>,
    pub FLAT_COLORED_FRAGMENT_SHADER: Arc<ShaderModule>,
}