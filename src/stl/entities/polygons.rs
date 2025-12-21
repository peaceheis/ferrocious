// use crate::core::entity::attributes::Attribute;
// use crate::core::entity::{Attribute, RenderedVertex};
// use crate::core::render::DefaultShaders;
// use crate::core::{Entity, Mutator, TimeStamp};
// use ndarray::Array3;
// use std::any::Any;
// use std::ops::Deref;
// use std::sync::Arc;
// use vulkano::shader::ShaderModule;
// 
// pub struct NGon {
//     num_points: usize,
//     start_frame: TimeStamp,
//     end_frame: TimeStamp,
//     positions: Vec<RenderedVertex>,
//     attributes: Vec<Box<dyn Attribute<AttributeType = ()>>>,
//     mutators: Vec<Box<dyn Mutator<AttributeType = ()>>>,
// }
// 
// impl NGon {
//     fn new(num_points: usize, start_frame: TimeStamp, end_frame: TimeStamp) {
//         Self {
//             num_points,
//             start_frame,
//             end_frame,
//             mutators: Vec::new(),
//             positions: Vec::new(),
//         };
//     }
// }
// 
// impl Entity for NGon {
//     fn tick(&mut self, frame: &TimeStamp) -> Vec<RenderedVertex> {
//         self.positions.clone()
//     }
//     fn tick_mutators(&mut self, frame: &TimeStamp) {
//         self.mutators
//             .iter_mut()
//             .filter(|m| m.is_active_at(frame))
//             .for_each(|m| m.tick(frame));
//     }
//     fn get_vertex_shader(&self, default_shaders: &DefaultShaders) -> Arc<ShaderModule> {
//         default_shaders.SIMPLE_VERTEX_SHADER.clone()
//     }
//     fn get_fragment_shader(&self, default_shaders: &DefaultShaders) -> Arc<ShaderModule> {
//         default_shaders.FLAT_COlORED_FRAGMENT_SHADER.clone()
//     }
// }
