use crate::core::mutator::timestamp::TimeStamp;
pub use ndarray::Array2;
use ndarray::Array3;
use video_rs::Frame;


pub trait Entity {
    fn render(&self, active_frame: &TimeStamp, fps: u8)-> Array3<u8>;
    fn get_size(&self) -> (u32, u32);
    fn is_active_at(&self, frame: &TimeStamp) -> bool;
    fn upper_left_coords(&self) -> (u32, u32);
    fn tick(&mut self, frame: &TimeStamp);
}
