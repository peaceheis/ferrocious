pub mod attributes;
pub mod bindings;
pub mod builder;

pub use ndarray::Array2;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::vertex_input::Vertex as VulkanoVertex;
use vulkano::shader::ShaderModule;
use crate::core::mutator::timestamp::TimeStamp;
use crate::core::render::DefaultShaders;

pub trait Entity {
    /// Returns the time ranges when this entity is visible
    /// If None, the entity is always visible
    fn active_range(&self) -> Option<Vec<[TimeStamp; 2]>> {
        None
    }

    /// Renders the entity at the given time, returning vertices
    /// This is a pure function - same time always produces same output
    fn render(&self, time: &TimeStamp, fps: u8) -> Vec<RenderedVertex>;

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

    fn render(&self, _time: &TimeStamp, _fps: u8) -> Vec<RenderedVertex> {
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
