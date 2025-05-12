use std::any::Any;
use ndarray::Array3;
use crate::core::{Entity, Mutator, TimeStamp};

struct NGon {
    num_points: u32,
    x_size: u32,
    y_size: u32,
    start_frame: TimeStamp,
    end_frame: TimeStamp,
    mutators: Vec<dyn Mutator>

}

impl Entity for NGon {
    fn render(&self, active_frame: &TimeStamp, fps: u32) -> Array3<u8> {
        todo!()
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.x_size, self.y_size)
    }

    fn is_active_at(&self, frame: &TimeStamp) -> bool {
        todo!()
    }

    fn upper_left_coords(&self) -> (u32, u32) {
        todo!()
    }

    fn tick(&mut self, frame: &TimeStamp) {
        todo!()
    }

    fn tick_mutators(&mut self, frame: &TimeStamp) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn as_any_mut(&self) -> &mut dyn Any {
        todo!()
    }
}

impl NGon {
    fn new() { todo!()}
}