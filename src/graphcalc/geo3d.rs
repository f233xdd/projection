mod component;
pub mod tool;
mod vector;
pub mod err;

pub use component::*;
pub use vector::SpaceVector;

pub type Geo3DResult<T> = Result<T, err::Geo3DError>;
