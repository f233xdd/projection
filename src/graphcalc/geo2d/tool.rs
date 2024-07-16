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

pub fn is_in(p: &Point, ln: &Line) -> bool {
    let k1 = ln.get_func_args()[0];
    let k2 = ln.get_func_args()[1];
    let b = ln.get_func_args()[2];
    k1 * p.x + k2 * p.y == b
}

/// superposition is not included
pub fn is_parallel(ln1: &Line, ln2: &Line) -> bool {
    let k11 = ln1.get_func_args()[0];
    let k12 = ln1.get_func_args()[1];
    let b1 = ln1.get_func_args()[2];
    let k21 = ln2.get_func_args()[0];
    let k22 = ln2.get_func_args()[1];
    let b2 = ln2.get_func_args()[2];
    (k11 * k22 == k21 * k12) && (k11 * b2 != k21 * b1)
}

pub fn is_vertical(ln1: &Line, ln2: &Line) -> bool {
    let k11 = ln1.get_func_args()[0];
    let k12 = ln1.get_func_args()[1];
    let k21 = ln2.get_func_args()[0];
    let k22 = ln2.get_func_args()[1];
    k11 * k21 + k12 * k22 == 0.0
}

pub fn point_is_superposition(p1: &Point, p2: &Point) -> bool {
    (p1.x == p2.x) && (p1.y == p2.y)
}

pub fn line_is_superposition(ln1: &Line, ln2: &Line) -> bool {
    let k11 = ln1.get_func_args()[0];
    let k12 = ln1.get_func_args()[1];
    let b1 = ln1.get_func_args()[2];
    let k21 = ln2.get_func_args()[0];
    let k22 = ln2.get_func_args()[1];
    let b2 = ln2.get_func_args()[2];
    k11 * b2 == k21 * b1 && k12 * b2 == k22 * b1
}

pub fn calc_point_d(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn calc_point_line_d(p: &Point, ln: &Line) -> f64 {
    let k1 = ln.get_func_args()[0];
    let k2 = ln.get_func_args()[1];
    let b = ln.get_func_args()[2];
    (k2 * p.x + k1 * p.y - b).abs() / (k1.powi(2) + k2.powi(2)).sqrt()
}

pub fn calc_line_d(ln1: &Line, ln2: &Line) -> Result<f64, ()> {
    let k11 = ln1.get_func_args()[0];
    let k12 = ln1.get_func_args()[1];
    let b1 = ln1.get_func_args()[2];
    let k21 = ln2.get_func_args()[0];
    let k22 = ln2.get_func_args()[1];
    let b2 = ln2.get_func_args()[2];
    if is_parallel(ln1, ln2) {
        let k = if k11 != 0.0 {k21 / k11} else {k22 / k12};
        Ok((k * b1 - b2).abs() / (k21.powi(2) + k22.powi(2)).sqrt())
    } else {Err(())}
}

pub fn calc_angle(ln1: &Line, ln2: &Line) -> f64 {
    let vec1 = ln1.get_direction_vec();
    let vec2 = ln2.get_direction_vec();
    ((&vec1 * &vec2).abs()/(vec1.len() * vec2.len())).acos()
}

pub fn calc_intersection(ln1: &Line, ln2: &Line) -> Result<Point, ()> {
    let k11 = ln1.get_func_args()[0];
    let k12 = ln1.get_func_args()[1];
    let b1 = ln1.get_func_args()[2];
    let k21 = ln2.get_func_args()[0];
    let k22 = ln2.get_func_args()[1];
    let b2 = ln2.get_func_args()[2];
    let x = (k12 * b1 - k11 * b2) / (k12 * k21 - k11 * k22);
    let y = (k21 * b1 - k12 * b2) / (k11 * k22 - k12 * k21);
    if k11 * k22 == k21 * k12 {Ok(Point{x, y})} else {Err(())}
}

pub mod feature {
    use super::super::component::Point;

    pub trait Inclusion<T> {
        fn is_included(&self, cpt: &T) -> bool;
    }

    pub trait Parallelism<T> {
        fn is_parallel(&self, cpt: &T) -> bool;
    }

    pub trait Vertical<T> {
        fn is_vertical(&self, cpt: &T) -> bool;
    }

    pub trait Superposition<T> {
        fn is_superposition(&self, cpt: &T) -> bool;
    }

    pub trait CalcDistance<T, U> {
        fn calc_d(&self, cpt: &T) -> U;
    }

    pub trait CalcAngle<T> {
        fn calc_angle(&self, cpt: &T) -> f64;
    }

    pub trait CalcIntersection<T> {
        fn calc_intersection(&self, cpt: &T) -> Result<Point, ()>;
    }
}
