use crate::entity::{Entity};
use crate::mutator::timestamp::TimeStamp;

pub trait Mutator {
    fn tick(&self, frame: &TimeStamp) -> &dyn Entity;
}
