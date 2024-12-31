// 2D part
use super::{
    component::{
        Point, Line,
    },
    super::{
        algebra::vector::PlaneVec,
        algebra::calc::approximate,
        geo_err,
    },
};

pub fn vec_to_line(vec: &PlaneVec, p: &Point) -> Result<Line, geo_err::InvalidFnArgError> {
    vec.to_line(p)
}

/// k1 * x + k2 * y = b
pub fn calc_line_fn(p1: &Point, p2: &Point) -> Result<(f64, f64, f64), geo_err::SuperpositionError> {
    let (x_p1, y_p1) = p1.pos();
    let (x_p2, y_p2) = p2.pos();
    if (x_p2 != x_p1) || (y_p2 != y_p1) {
        return Ok((y_p1 - y_p2, x_p2 - x_p1, x_p2 * y_p1 - x_p1 * y_p2));
    } else {
        return Err(geo_err::SuperpositionError());
    }
}


pub fn is_in(p: &Point, ln: &Line) -> bool {
    let (x_p, y_p) = p.pos();
    let (k1, k2, b) = ln.fn_args();
    approximate(k1 * x_p + k2 * y_p, b)
}

/// superposition is not included
pub fn is_parallel(ln1: &Line, ln2: &Line) -> bool {
    let (k11, k12, b1) = ln1.fn_args();
    let (k21, k22, b2) = ln2.fn_args();
    approximate(k11 * k22, k21 * k12) &&
    !approximate(k11 * b2, k21 * b1)
}

pub fn is_vertical(ln1: &Line, ln2: &Line) -> bool {
    let (k11, k12, _) = ln1.fn_args();
    let (k21, k22, _) = ln2.fn_args();
    approximate(k11 * k21 + k12 * k22, 0.0)
}

pub fn point_is_superposition(p1: &Point, p2: &Point) -> bool {
    let (x_p1, y_p1) = p1.pos();
    let (x_p2, y_p2) = p2.pos();
    approximate(x_p1, x_p2) &&
    approximate(y_p1, y_p2)
}

pub fn line_is_superposition(ln1: &Line, ln2: &Line) -> bool {
    let (k11, k12, b1) = ln1.fn_args();
    let (k21, k22, b2) = ln2.fn_args();
    approximate(k11 * b2, k21 * b1) &&
    approximate(k12 * b2, k22 * b1)
}

pub fn calc_point_d(p1: &Point, p2: &Point) -> f64 {
    let (x_p1, y_p1) = p1.pos();
    let (x_p2, y_p2) = p2.pos();
    ((x_p1 - x_p2).powi(2) + (y_p1 - y_p2).powi(2)).sqrt()
}

pub fn calc_point_line_d(p: &Point, ln: &Line) -> f64 {
    let (x_p, y_p) = p.pos();
    let (k1, k2, b) = ln.fn_args();
    (k1 * x_p + k2 * y_p - b).abs() / (k1.powi(2) + k2.powi(2)).sqrt()
}

pub fn calc_line_d(ln1: &Line, ln2: &Line) -> Result<f64, geo_err::NotParallelError> {
    let (k11, k12, b1) = ln1.fn_args();
    let (k21, k22, b2) = ln2.fn_args();
    if is_parallel(ln1, ln2) {
        let k = if k11 != 0.0 {k21 / k11} else {k22 / k12};
        Ok((k * b1 - b2).abs() / (k21.powi(2) + k22.powi(2)).sqrt())
    } else {Err(geo_err::NotParallelError())}
}

pub fn calc_angle(ln1: &Line, ln2: &Line) -> f64 {
    let vec1 = ln1.get_direction_vec();
    let vec2 = ln2.get_direction_vec();
    ((&vec1 * &vec2).abs()/(vec1.norm() * vec2.norm())).acos()
}

pub fn calc_intersection(ln1: &Line, ln2: &Line) -> Result<Point, geo_err::ParallelError> {
    let (k11, k12, b1) = ln1.fn_args();
    let (k21, k22, b2) = ln2.fn_args();
    let v = k11 * k22 - k21 * k12;
    if v != 0.0 {
        let x = (k21 * b1 - k12 * b2) / v;
        let y = (k12 * b1 - k11 * b2) / -v;
        Ok(Point::new(x, y))
    } else {Err(geo_err::ParallelError())}
}

/// theta in radians
pub fn rotate(o: &Point, p: &mut Point, theta: f64) {
    let (x_p, y_p) = p.pos();
    let (x_o, y_o) = o.pos();
    p.move_to(
        x_p * theta.cos() - y_p * theta.sin() + x_o,
        x_p * theta.sin() + y_p * theta.cos() + y_o
    );
}