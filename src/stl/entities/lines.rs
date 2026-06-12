use crate::core::entity::{Entity, Point, PushConstantData, RenderedVertex, UniformData};
use crate::core::geometry::create_stroke;
use crate::core::interpolate::Interpolator;
use crate::core::mutator::timestamp::TimeStamp;
use crate::core::render::pipeline_layouts::TrigLineConstants;
use crate::core::render::DefaultShaders;
use std::f32::consts::PI;
use std::marker::PhantomData;
use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::pipeline::graphics::input_assembly::PrimitiveTopology;
use vulkano::pipeline::{GraphicsPipeline, Pipeline};
use vulkano::shader::ShaderModule;

const DEFAULT_AMPLITUDE: f32 = 1.;
const DEFAULT_FREQUENCY: f32 = 2. * PI;
const DEFAULT_PHASE: f32 = 0.;
const DEFAULT_ORIENTATION: f32 = 0.;
const DEFAULT_START_POINT: Point = [0., 0.];
const DEFAULT_THICKNESS: f32 = 1.;
const DEFAULT_COLOR: [f32; 4] = [1., 1., 1., 1.];
const DEFAULT_RADIUS: f32 = 1.;
const DEFAULT_RADIANS: f32 = 2. * PI;
const DEFAULT_START_RADIANS: f32 = 0.;

pub struct NeedsStart;
pub struct NeedsEnd;
pub struct Bounded;

// TODO: do resolution-based adjustment for thickness

#[derive(Clone, Debug)]
pub struct ArcLine {
    pub center: Interpolator<Point>,
    pub radius: Interpolator<f32>,
    pub radians: Interpolator<f32>,
    pub start_radians: Interpolator<f32>,
    pub thickness: Interpolator<f32>,
}

impl ArcLine {
    pub fn new(
        center: Interpolator<Point>,
        radius: Option<Interpolator<f32>>,
        radians: Option<Interpolator<f32>>,
        start_radians: Option<Interpolator<f32>>,
        thickness: Option<Interpolator<f32>>,
    ) -> Self {
        ArcLine {
            center,
            radius: radius.unwrap_or(DEFAULT_RADIUS.into()),
            radians: radians.unwrap_or(DEFAULT_RADIANS.into()),
            start_radians: start_radians.unwrap_or(DEFAULT_START_RADIANS.into()),
            thickness: thickness.unwrap_or(DEFAULT_THICKNESS.into()),
        }
    }
}

impl Entity for ArcLine {
    fn render(&self, time: &TimeStamp, fps: u32, viewport: [u32; 2]) -> Vec<RenderedVertex> {
        todo!()
    }

    fn get_vertex_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        todo!()
    }

    fn get_fragment_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct LinearLine {
    pub start: Interpolator<Point>,
    pub end: Interpolator<Point>,
    pub thickness: Interpolator<f32>,
    pub color: Interpolator<[f32; 4]>
}

impl LinearLine {
    pub fn new(
        start: Interpolator<Point>,
        end: Interpolator<Point>,
        thickness: Option<Interpolator<f32>>,
        color: Option<Interpolator<[f32; 4]>>
    ) -> Self {
        Self {
            start,
            end,
            thickness: thickness.unwrap_or(DEFAULT_THICKNESS.into()),
            color: color.unwrap_or(DEFAULT_COLOR.into())
        }
    }
}

impl Entity for LinearLine {
    fn render(&self, time: &TimeStamp, fps: u32, viewport: [u32; 2]) -> Vec<RenderedVertex> {
        let start = self.start.at(time, fps);
        let end = self.end.at(time, fps);
        let thickness = self.thickness.at(time, fps);
        let color = self.color.at(time, fps);

        create_stroke(&[start, end], thickness, viewport)
            .into_iter()
            .map(|position| RenderedVertex { position, color })
            .collect()
    }
}

/// Polynomial line of degree `degree.len()` coefficients `coefficients`, and roots `roots`
/// Consider the potential input and output spaces as `[-1, 1]` (Ferrocious screen space, can be
/// aligned otherwise, but will be off screen.
/// the start point becomes the center of this input space (so default is thus (0, 0))
/// orientation rotates this line (x-axis becomes ray centered at start point, with angle given by orientation)
/// offset moves the line along the ray (`x-h` style)
#[derive(Clone, Debug)]
pub struct PolynomialLine {
    pub coefficients: Vec<Interpolator<f32>>,
    pub roots: Vec<Interpolator<Point>>,
    pub offsets: Vec<Interpolator<f32>>,
    pub start_point: Interpolator<Point>,
    pub input_start: Interpolator<f32>,
    pub input_end: Interpolator<f32>,
    pub offset: Interpolator<f32>,
    pub orientation: Interpolator<f32>,
    pub thickness: Interpolator<f32>,
}

impl PolynomialLine {
    pub fn with_degree(degree: usize) -> PolynomialLineBuilder {
        PolynomialLineBuilder {
            degree,
            coefficients: None,
            roots: None,
            offsets: None,
            start_point: None,
            input_start: None,
            input_end: None,
            offset: None,
            thickness: None,
        }
    }

    pub fn default_with_degree(degree: usize) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct PolynomialLineBuilder {
    degree: usize,
    coefficients: Option<Interpolator<f32>>,
    roots: Option<Vec<Interpolator<f32>>>,
    offsets: Option<Vec<Interpolator<f32>>>,
    start_point: Option<Interpolator<Point>>,
    input_start: Option<Interpolator<f32>>,
    input_end: Option<Interpolator<f32>>,
    offset: Option<Interpolator<f32>>,
    thickness: Option<Interpolator<f32>>,
}

pub struct SplineLine;

#[derive(Debug, Clone)]
enum TrigType {
    COS,
    SIN,
    TAN,
    ARCCOS,
    ARCSIN,
    ARCTAN,
    SEC,
    COSEC,
    COTAN,
    ARCSEC,
    ARCCOSEC,
    ARCCOTAN,
}

//TODO: default input_angle_start, input_angle_end?

/// Trigonometric line. see `TrigType` for options.
/// Input is in radians space, output is in screen space (`[-1, 1]`).
/// Calculates `func(θ*frequency + phase)*amplitude`, over `input_angle_start`
/// to `input_angle_end`, starting at start_point in ferrocious screen space, and rotated `orientation` radians
#[derive(Clone, Debug)]
pub struct TrigLine {
    pub type_: TrigType,
    pub input_angle_start: Interpolator<f32>,
    pub input_angle_end: Interpolator<f32>,
    pub amplitude: Interpolator<f32>,
    pub frequency: Interpolator<f32>,
    pub phase: Interpolator<f32>,
    pub orientation: Interpolator<f32>,
    pub start_point: Interpolator<Point>,
    pub thickness: Interpolator<f32>,
}

impl TrigLine {
    pub fn sin() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::SIN,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn cos() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::COS,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn tan() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::TAN,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn arccos() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::ARCCOS,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn arcsin() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::ARCSIN,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn arctan() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::ARCTAN,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn sec() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::SEC,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn cosec() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::COSEC,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn cotan() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::COTAN,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn arcsec() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::ARCSEC,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn arccosec() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::ARCCOSEC,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
    pub fn arccotan() -> TrigLineBuilder<NeedsStart> {
        TrigLineBuilder {
            type_: TrigType::ARCCOTAN,
            _state: PhantomData,
            input_angle_start: None,
            input_angle_end: None,
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
        }
    }
}

impl Entity for TrigLine {
    fn uses_gpu_computation(&self) -> bool {
        true
    }

    fn get_gpu_vertex_count(&self, time: &TimeStamp, fps: u32) -> u32 {
        let resolution = self.calculate_resolution(time, fps);
        // 4 vertices per segment (quad as 2 triangles in triangle strip)
        resolution * 4
    }

    fn get_topology(&self) -> PrimitiveTopology {
        PrimitiveTopology::TriangleStrip
    }

    fn render(&self, _time: &TimeStamp, _fps: u32, _viewport: [u32; 2]) -> Vec<RenderedVertex> {
        // Not used for GPU computation
        vec![]
    }

    fn bind_resources(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        pipeline: &Arc<GraphicsPipeline>,
        time: &TimeStamp,
        fps: u32,
    ) {
        // Evaluate all interpolated parameters at current time
        let start_point = self.start_point.at(time, fps);

        let constants = TrigLineConstants {
            amplitude: self.amplitude.at(time, fps),
            frequency: self.frequency.at(time, fps),
            phase: self.phase.at(time, fps),
            thickness: self.thickness.at(time, fps),
            trig_type: self.type_.to_u32(),
            start_point_x: start_point[0],
            start_point_y: start_point[1],
            orientation: self.orientation.at(time, fps),
            _padding1: 0.0,
            input_angle_start: self.input_angle_start.at(time, fps),
            input_angle_end: self.input_angle_end.at(time, fps),
            resolution: self.calculate_resolution(time, fps),
            _padding2: 0,
            color_r: 1.0, // TODO: Add color interpolation
            color_g: 1.0,
            color_b: 1.0,
            color_a: 1.0,
        };

        builder
            .push_constants(pipeline.layout().clone(), 0, constants)
            .unwrap();
    }

    fn get_vertex_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        defaults.TRIG_LINE_VERTEX.clone()
    }

    fn get_fragment_shader(&self, defaults: &DefaultShaders) -> Arc<ShaderModule> {
        defaults.TRIG_LINE_FRAGMENT.clone()
    }
}

impl TrigLine {
    fn calculate_resolution(&self, _time: &TimeStamp, _fps: u32) -> u32 {
        // TODO: Base on viewport size and input range
        // For now, use a fixed resolution
        1200
    }
}

impl TrigType {
    fn to_u32(&self) -> u32 {
        match self {
            TrigType::SIN => 0,
            TrigType::COS => 1,
            TrigType::TAN => 2,
            TrigType::ARCSIN => 3,
            TrigType::ARCCOS => 4,
            TrigType::ARCTAN => 5,
            TrigType::SEC => 6,
            TrigType::COSEC => 7,
            TrigType::COTAN => 8,
            TrigType::ARCSEC => 9,
            TrigType::ARCCOSEC => 10,
            TrigType::ARCCOTAN => 11,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrigLineBuilder<State> {
    type_: TrigType,
    input_angle_start: Option<Interpolator<f32>>,
    input_angle_end: Option<Interpolator<f32>>,
    amplitude: Option<Interpolator<f32>>,
    frequency: Option<Interpolator<f32>>,
    phase: Option<Interpolator<f32>>,
    orientation: Option<Interpolator<f32>>,
    start_point: Option<Interpolator<Point>>,
    thickness: Option<Interpolator<f32>>,
    _state: PhantomData<State>,
}

impl TrigLineBuilder<NeedsStart> {
    pub fn start(self, start: Interpolator<f32>) -> TrigLineBuilder<NeedsEnd> {
        TrigLineBuilder {
            type_: self.type_,
            input_angle_start: Some(start),
            input_angle_end: self.input_angle_end,
            amplitude: self.amplitude,
            frequency: self.frequency,
            phase: self.phase,
            orientation: self.orientation,
            start_point: self.start_point,
            thickness: self.thickness,
            _state: PhantomData,
        }
    }
}

impl TrigLineBuilder<NeedsEnd> {
    pub fn end(self, end: Interpolator<f32>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            type_: self.type_,
            input_angle_start: self.input_angle_start,
            input_angle_end: Some(end),
            amplitude: None,
            frequency: None,
            phase: None,
            orientation: None,
            start_point: None,
            thickness: None,
            _state: PhantomData,
        }
    }
}

impl TrigLineBuilder<Bounded> {
    pub fn amplitude(self, amplitude: Interpolator<f32>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            amplitude: Some(amplitude),
            ..self
        }
    }

    pub fn frequency(self, frequency: Interpolator<f32>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            frequency: Some(frequency),
            ..self
        }
    }

    pub fn phase(self, phase: Interpolator<f32>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            phase: Some(phase),
            ..self
        }
    }

    pub fn orientation(self, orientation: Interpolator<f32>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            orientation: Some(orientation),
            ..self
        }
    }

    pub fn start_point(self, start_point: Interpolator<Point>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            start_point: Some(start_point),
            ..self
        }
    }

    pub fn thickness(self, thickness: Interpolator<f32>) -> TrigLineBuilder<Bounded> {
        TrigLineBuilder {
            thickness: Some(thickness),
            ..self
        }
    }

    pub fn build(self) -> TrigLine {
        TrigLine {
            type_: self.type_,
            // Bounded, so unwrap is safe
            input_angle_start: self.input_angle_start.unwrap(),
            input_angle_end: self.input_angle_end.unwrap(),
            amplitude: self.amplitude.unwrap_or(DEFAULT_AMPLITUDE.into()),
            frequency: self.frequency.unwrap_or(DEFAULT_FREQUENCY.into()),
            phase: self.phase.unwrap_or(DEFAULT_PHASE.into()),
            orientation: self.orientation.unwrap_or(DEFAULT_ORIENTATION.into()),
            start_point: self.start_point.unwrap_or(DEFAULT_START_POINT.into()),
            thickness: self.thickness.unwrap_or(DEFAULT_THICKNESS.into()),
        }
    }
}

pub struct BezierLine;
