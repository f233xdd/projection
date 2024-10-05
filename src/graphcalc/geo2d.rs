mod component;
pub mod tool;
mod vector;
pub mod err;

pub use component::*;
pub use vector::PlaneVector;

pub type Geo2DResult<T> = Result<T, err::Geo2DError>;
