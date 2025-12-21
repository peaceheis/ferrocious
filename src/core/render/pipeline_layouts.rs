use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct PushConstants {
    pub time: u32,
    pub fps: u32,
}
