pub mod attributes;
pub mod bindings;
pub mod builder;

use crate::core::mutator::timestamp::TimeStamp;
use crate::core::render::DefaultShaders;
pub use ndarray::Array2;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::input_assembly::PrimitiveTopology;
use vulkano::pipeline::graphics::vertex_input::Vertex as VulkanoVertex;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::shader::ShaderModule;

pub type Point = [f32; 2];

pub enum UniformType {
    Buffer(Vec<u8>),
}

pub enum ShaderStages {
    VERTEX,
    FRAGMENT,
    BOTH,
}

pub struct PushConstantData {
    pub bytes: Vec<u8>,
    pub stage_flags: ShaderStages,
}

pub struct UniformData {
    pub binding: u32,
    pub data: UniformType,
}

pub trait Entity {
    /// Returns the time ranges when this entity is visible
    /// If None, the entity is always visible
    fn active_range(&self) -> Option<Vec<[TimeStamp; 2]>> {
        None
    }

    /// Renders the entity at the given time, returning vertices
    /// This is a pure function - same time always produces same output
    fn render(&self, time: &TimeStamp, fps: u32, viewport: [u32; 2]) -> Vec<RenderedVertex>;

    fn get_push_constants(&self, time: &TimeStamp, fps: u32, viewport: [u32; 2]) -> Option<PushConstantData> {
        None
    }
    fn get_uniforms(&self, time: &TimeStamp, fps: u32, viewport: [u32; 2]) -> Option<UniformData> {
        None
    }

    /// Get vertex shader (override for custom shaders)
    fn get_vertex_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        defaults.SIMPLE_VERTEX_SHADER.clone()
    }

    /// Get fragment shader (override for custom shaders)
    fn get_fragment_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        defaults.FLAT_COLORED_FRAGMENT_SHADER.clone()
    }

    /// Get blending mode (override for custom blending)
    fn get_blending(&self) -> ColorBlendAttachmentState {
        ColorBlendAttachmentState {
            blend: None,
            ..Default::default()
        }
    }

    /// Returns whether this entity uses GPU-based computation
    /// If true, render() is not called and GPU generates geometry from push constants/uniforms
    fn uses_gpu_computation(&self) -> bool {
        false // Default: CPU-side vertex generation
    }

    /// Returns vertex count for GPU-computed geometry
    /// Only called if uses_gpu_computation() returns true
    fn get_gpu_vertex_count(&self, _time: &TimeStamp, _fps: u32) -> u32 {
        0
    }

    /// Returns the primitive topology for this entity
    fn get_topology(&self) -> PrimitiveTopology {
        PrimitiveTopology::TriangleList // Default
    }

    /// Bind resources (push constants, descriptor sets) to the command buffer
    /// This is called after the pipeline is bound but before drawing
    /// Entities that need push constants or uniforms should override this method
    fn bind_resources(
        &self,
        _builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        _pipeline: &Arc<GraphicsPipeline>,
        _time: &TimeStamp,
        _fps: u32,
    ) {
        // Default: no resources to bind
    }
}

#[derive(BufferContents, VulkanoVertex)]
#[repr(C)]
#[derive(Clone)]
pub struct RenderedVertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],

    #[format(R32G32B32A32_SFLOAT)]
    pub color: [f32; 4],
}

pub struct PlainEntity {
    pub vertices: Vec<RenderedVertex>,
    pub active_ranges: Option<Vec<[TimeStamp; 2]>>,
    vertex_shader: Option<Arc<ShaderModule>>,
    fragment_shader: Option<Arc<ShaderModule>>,
}

impl PlainEntity {
    pub fn new(vertices: Vec<RenderedVertex>) -> Self {
        Self {
            vertices,
            active_ranges: None,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn with_active_ranges(mut self, ranges: Vec<[TimeStamp; 2]>) -> Self {
        self.active_ranges = Some(ranges);
        self
    }
}

impl Entity for PlainEntity {
    fn active_range(&self) -> Option<Vec<[TimeStamp; 2]>> {
        self.active_ranges.clone()
    }

    fn render(&self, _time: &TimeStamp, _fps: u32, _viewport: [u32; 2]) -> Vec<RenderedVertex> {
        // PlainEntity just returns static vertices (doesn't change over time)
        self.vertices.clone()
    }

    fn get_vertex_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        self.vertex_shader
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| defaults.SIMPLE_VERTEX_SHADER.clone())
    }

    fn get_fragment_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        self.fragment_shader
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| defaults.FLAT_COLORED_FRAGMENT_SHADER.clone())
    }
}
