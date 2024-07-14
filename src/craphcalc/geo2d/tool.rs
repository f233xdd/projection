use crate::craphcalc::geo2d::component::*;

pub fn calc_d(p1: Point, p2: Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

