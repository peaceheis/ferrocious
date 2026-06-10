use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct PushConstants {
    pub time: u32,
    pub fps: u32,
}

/// Push constants for TrigLine shader
/// Total size: 68 bytes (well under 128 byte limit)
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TrigLineConstants {
    // Function parameters (20 bytes)
    pub amplitude: f32,
    pub frequency: f32,
    pub phase: f32,
    pub thickness: f32,
    pub trig_type: u32, // 0=SIN, 1=COS, 2=TAN, etc.

    // Spatial parameters (16 bytes)
    pub start_point_x: f32,
    pub start_point_y: f32,
    pub orientation: f32,
    pub _padding1: f32,

    // Range parameters (16 bytes)
    pub input_angle_start: f32,
    pub input_angle_end: f32,
    pub resolution: u32,
    pub _padding2: u32,

    // Color parameters (16 bytes)
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
}

/// Push constants for PolynomialLine shader
/// Total size: 56 bytes
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct PolynomialLineConstants {
    // Spatial parameters (16 bytes)
    pub start_point_x: f32,
    pub start_point_y: f32,
    pub orientation: f32,
    pub offset: f32,

    // Range parameters (16 bytes)
    pub input_start: f32,
    pub input_end: f32,
    pub thickness: f32,
    pub resolution: u32,

    // Color parameters (16 bytes)
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,

    // Metadata (8 bytes)
    pub num_coefficients: u32,
    pub _padding: u32,
}
