pub mod capabilities;

use std::any::Any;
use crate::core::mutator::timestamp::TimeStamp;
pub use ndarray::Array2;
use ndarray::Array3;
use video_rs::Frame;
use crate::core::entity::capabilities::sizeable::Sizeable;
use crate::core::Mutator;


pub trait Entity: Any {
    fn render(&self, active_frame: &TimeStamp, fps: u8)-> Array3<u8>;
    fn get_dimensions(&self) -> (u32, u32);
    fn is_active_at(&self, frame: &TimeStamp) -> bool;
    fn upper_left_coords(&self) -> (u32, u32);
    fn tick(&mut self, frame: &TimeStamp);
    fn tick_mutators(&mut self, frame: &TimeStamp);
    fn get_mutators(&self) -> &Vec<Box<dyn Mutator>>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&self) -> &mut dyn Any;
    fn get_as_sizeable(&self) -> Option<&mut dyn Sizeable> {
        return None
    }
}
