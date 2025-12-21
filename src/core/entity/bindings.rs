// use std::any::Any;
// use crate::core::entity::attributes::Attribute;
// use crate::core::entity::{SharedAttribute, SharedMutator};
// use crate::core::{Mutator, TimeStamp};
// use std::cell::RefCell;
// use std::ops::DerefMut;
// use std::rc::Rc;
// 
// pub struct TimedAttributeMutatorBinding {
//     time_intervals: Vec<[TimeStamp; 2]>,
//     pub attributes: Vec<SharedAttribute>,
//     pub mutators: Vec<SharedMutator>,
// }
// 
// impl TimedAttributeMutatorBinding {
//     pub fn is_active(&self, time: &TimeStamp) -> bool {
//         time.in_range(&self.time_intervals)
//     }
// 
//     pub fn tick(&mut self, frame: &TimeStamp) {
//         self.mutators.iter_mut().for_each(|mutator| {
//             self.attributes.iter_mut().for_each(|attribute| {
//                 (*mutator.borrow_mut()).tick(&mut *attribute.borrow_mut(), frame);
//             })
//         })
//     }
// }
// 
// pub struct TimedAttributeMutatorBindingBuilder {
//     time_intervals: Vec<[TimeStamp; 2]>,
//     attributes: Vec<SharedAttribute>,
//     mutators: Vec<SharedMutator>,
// }
// 
// impl<'a> TimedAttributeMutatorBindingBuilder {
//     pub fn new() -> Self {
//         Self {
//             time_intervals: Vec::new(),
//             attributes: Vec::new(),
//             mutators: Vec::new(),
//         }
//     }
// 
//     pub fn add_time_interval(&mut self, time_interval: [TimeStamp; 2]) {
//         self.time_intervals.push(time_interval);
//     }
// 
//     pub fn add_attribute(&mut self, attribute: Box<dyn Attribute<AttributeType = ()>>) {
//         self.attributes.push(Rc::new(RefCell::new(attribute)));
//     }
// 
//     pub fn add_mutator(&mut self, mutator: Box<dyn Mutator<AttributeTypeType=(), AttributeType=()>>) {
//         self.mutators.push(Rc::new(RefCell::new(mutator)));
//     }
// 
//     pub fn build(self) -> TimedAttributeMutatorBinding {
//         TimedAttributeMutatorBinding {
//             time_intervals: self.time_intervals,
//             attributes: self.attributes,
//             mutators: self.mutators,
//         }
//     }
// }
