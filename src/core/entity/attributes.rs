// use std::ops::Deref;
// use crate::core::entity::attributes;
// use crate::core::{Mutator, TimeStamp};
// 
// 
// pub struct Position {
//     pub x: f32,
//     pub y: f32,
// }
// 
// pub enum PositionIndex {
//     X = 0,
//     Y = 1,
// }
// 
// impl Position {
//     pub fn new(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }
// 
//     pub fn rotate(
//         &mut self,
//         theta: f32,
//         &Position {
//             x: center_x,
//             y: center_y,
//         }: &Position,
//     ) {
//         let (x, y) = (self.x - center_x, self.y - center_y);
//         self.x = center_x + x * theta.cos() - y * theta.sin();
//         self.y = center_y + x * theta.sin() + y * theta.cos();
//     }
// }
// 
// 
// 
// impl From<Position> for [f32; 2] {
//     fn from(position: Position) -> Self {
//         [position.x, position.y]
//     }
// }
// 
// pub struct RGBA {
//     pub r: f32,
//     pub g: f32,
//     pub b: f32,
//     pub a: f32,
// }
// 
// pub enum RGBAIndex {
//     RED = 0,
//     GREEN = 1,
//     BLUE = 2,
//     ALPHA = 3,
// }
// 
// 
// impl From<RGBA> for [f32; 4] {
//     fn from(rgba: RGBA) -> Self {
//         [rgba.r, rgba.g, rgba.b, rgba.a]
//     }
// }
