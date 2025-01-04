pub mod algebra;
pub mod geo2d;
pub mod geo3d;
pub mod geo_err;
pub mod interface;
mod macros;

type GeoResult<T> = Result<T, geo_err::GeoError>;
