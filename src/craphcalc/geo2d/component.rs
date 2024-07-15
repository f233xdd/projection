// 2D part
use super::tool::{calc_line_func, };


pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}


/// function sample:
///     k1 * y+ k2 * x = b
pub struct Line {
    func_args: [f64;3]
}


impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Result<Line, ()> {
        match calc_line_func(p1, p2) {
            Ok(func_args) => {Ok(Line{func_args})}
            Err(e) => {Err(())}
        }
    }
    
    /// k1 * y+ k2 * x = b
    pub fn from(k1: f64, k2: f64, b: f64) -> Result<Line, ()> {
        if k1 == 0.0 && k2 == 0.0{
            Err(())
        } else {
            Ok(Self{func_args: [k1, k2, b]})
        }
    } 
}

