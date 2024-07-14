// 2D part
use super::component::{Point, Line};

/// function sample:
///     k1*y+ k2*x = b
pub fn calc_line_func(p1: &Point, p2: &Point) -> Option<[f64; 3]> {
    if (p2.x != p1.x) || (p2.y != p1.y) {
        return Some([p2.x - p1.x, p1.y - p2.y, p2.x * p1.y - p1.x * p2.y]);
    } else {
        return None;
    }
}

pub fn calc_d(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

