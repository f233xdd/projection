use super::tool::{calc_line_func, calc_plane_func};

// 3D part
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {x, y, z}
    }
}


pub struct Line {
    pub func_args: [[f64; 3]; 2]
}


impl Line {
    fn new(p1: &Point, p2: &Point) -> Option<Line> {
        match calc_line_func(p1, p2) {
            Some(func_args) => {Some(Line{func_args})}
            None => {None}
        }
    }
}


pub struct Plane  {
    pub func_args: [f64; 4]
}


impl Plane {
    fn new(p1: &Point, p2: &Point, p3: &Point) -> Option<Plane>  {
        match calc_plane_func(p1, p2, p3) {
            Some(func_args) => {Some(Plane{func_args})}
            None => {None}
        }
    }
}