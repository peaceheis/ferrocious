use crate::mutator::timestamp::TimeStamp;

pub trait Entity {
    fn render(&self, active_frame: &TimeStamp, fps: u32)-> ndarray::Array2<u32>;
    fn get_size(&self) -> (u32, u32);
    fn is_active_at(&self, frame: &TimeStamp) -> bool;
    fn upper_left_coords(&self) -> (u32, u32);
    fn tick(&mut self, frame: &TimeStamp);
}
