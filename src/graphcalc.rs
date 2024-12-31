pub mod geo2d;
pub mod geo3d;
pub mod algebra;
pub mod interface;
pub mod geo_err;
mod macros;

type GeoResult<T> = Result<T, geo_err::GeoError>;