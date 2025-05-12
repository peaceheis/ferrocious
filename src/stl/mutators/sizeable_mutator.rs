use thiserror::Error;
use crate::core::{Entity, Mutator, TimeStamp};
use crate::core::entity::capabilities::sizeable::{Size, Sizeable};

#[non_exhaustive]
pub struct SizeableMutator {
    target_dimension: usize,
    end_size: f32,
    interpolation_func: Box<dyn Fn(TimeStamp) -> f64>
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SizeableMutatorCreationError {
    // value represents number of dimensions of Entity
    #[error("Attempted to create SizeableMutator targeting dimension {target}, but entity's size has only {entity_dim_size} dimensions.")]
    InvalidDimensionError{target: usize, entity_dim_size: usize}
}

impl SizeableMutator {
    fn new(target_dimension: usize, size_type: Size, end_size: f32, interpolation_func: Box<dyn Fn()>) -> Result<SizeableMutator, SizeableMutatorCreationError> {
        if target_dimension >= size_type.dim_count() {
            return Err(SizeableMutatorCreationError::InvalidDimensionError {target: target_dimension, entity_dim_size: size_type.dim_count()})
        }
        return Ok(SizeableMutator {
            target_dimension,
            end_size,
            interpolation_func
        });
    }
 }

impl Mutator for SizeableMutator {
    fn tick(&self, entity: &mut dyn Entity, frame: &TimeStamp) {
        if let Some(sized_entity) = entity.as_any_mut().downcast_mut::<dyn Sizeable>() {
            entity.get_
        }
    }
}