use crate::core::entity::{Entity};
use crate::core::mutator::timestamp::TimeStamp;

pub trait Mutator {
    fn tick(&self, frame: &TimeStamp) -> &dyn Entity;
}
