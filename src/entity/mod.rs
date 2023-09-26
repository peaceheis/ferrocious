use crate::mutator::timestamp::TimeStamp;

use ndarray::prelude::*;

pub trait Entity {
    fn render(&self, active_frame: &TimeStamp, fps: u8)-> &Array<Array<Array<u32, u8>, u16>, u16>;
    fn get_size(&self) -> (u16, u16);
}
