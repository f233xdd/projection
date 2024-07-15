// 2D part
use super::component::{Point, Line};


/// k1 * y+ k2 * x = b
pub fn calc_line_func(p1: &Point, p2: &Point) -> Result<[f64; 3], ()> {
    if (p2.x != p1.x) || (p2.y != p1.y) {
        return Ok([p2.x - p1.x, p1.y - p2.y, p2.x * p1.y - p1.x * p2.y]);
    } else {
        return Err(());
    }
}

pub fn calc_d(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn is_in(p: &Point, ln: &Line) -> bool {}
pub fn is_parallel(ln1: &Line, ln2: &Line) -> bool {}
pub fn is_vertical(ln1: &Line, ln2: &Line) -> bool {}
pub fn point_is_superposition(p1: &Point, p2: &Point) -> bool {}
pub fn line_is_superposition(ln1: &Line, ln2: &Line) -> bool {}

pub fn calc_point_d() -> f64 {}
pub fn calc_point_line_d() -> f64 {}
pub fn calc_angle() -> f64 {}

pub trait Inclusion<T> {
    fn is_included(&self, cpt: T) -> bool;
}
pub trait Parallelism<T> {
    fn is_parallel(&self, cpt: T) -> bool;
}
pub trait Vertical<T> {
    fn is_vertical(&self, cpt: T) -> bool;
}
pub trait Superposition<T> {
    fn is_superposition(&self, cpt: T) -> bool;
}  
pub trait CalcDistance<T> {
    fn calc_d(&self, cpt: T) -> f64;
}
pub trait CalcAngle<T> {
    fn calc_angle(&self, cpt: T) -> f64;
}
pub trait CalcIntersection<T> {
    fn calc_intersection(&self, cpt: T) -> Result<Point, ()>;
}