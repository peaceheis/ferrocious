use std::any::Any;
use thiserror::Error;
use crate::core::Entity;

#[non_exhaustive]
pub enum Size {
    Scalar{ scalar: f32},
    XY{x: f32, y: f32},
    Custom{vec: Vec<f32>}
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum SizeError {
    #[error(Tried to change value at dimension {dimension}, but there are only {dim_count} dimensions)]
    SetSizeError{dimension: usize, dim_count: usize},
    #[error(Tried to retrieve value at dimension {dimension}, but there are only {dim_count} dimensions)]
    GetSizeError{dimension: usize, dim_count: usize}
}

impl Size {
    pub fn dim_count(&self) -> usize{
        return match self {
            Size::Scalar => 1,
            Size::XY => 2,
            Size::Custom{vec} => vec.len()
        }
    }

    pub fn change_size_at_dim(&mut self, dim: usize, val: f32) -> Result<(), SizeError> {
        if self.dim_count() <= dim {
            return Err(SizeError::SetSizeError {dimension: dim, dim_count: self.dim_count()})
        } else {
            match self {
                Size::Scalar {ref mut scalar} => {*scalar=val},
                Size::XY {ref mut x, ref mut y} => {if dim == 0 {*x=val} else {*y=val}}
                Size::Custom{ref mut vec} => {vec[dim] = val}
            }
        };
        return Ok(())
    }

    pub fn get_size_at_dim(&self, dim: usize) -> Result<f32, SizeError> {
        if self.dim_count() <= dim {
            return Err(SizeError::GetSizeError{dimension: dim, dim_count: self.dim_count()})
        }
        let &return_val= match self {
                Size::Scalar {scalar} => {scalar},
                Size::XY {ref mut x, ref mut y} => {if dim == 0 {x} else {y}}
                Size::Custom{ref mut vec} => &{ vec[dim] }
        };

        return Ok(return_val)

    }
}

pub trait Sizeable: Entity + Any {
    fn get_size(&self) -> Size;
    fn get_as_sizeable(&self) -> Option<&mut dyn Sizeable> {
        return Some(&self)
    }
}