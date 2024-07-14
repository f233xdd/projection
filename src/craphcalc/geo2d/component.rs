// 2D part
use super::tool::{calc_line_func, };


pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}


pub struct Line {
    pub func_args: [f64;3]
}


impl Line {
    fn new(p1: &Point, p2: &Point) -> Option<Line> {
        match calc_line_func(p1, p2) {
            Some(func_args) => {Some(Line{func_args})}
            None => {None}
        }
    }
}

