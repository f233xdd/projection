use std::fmt;

// 2D part
use super::tool::*;
use super::tool::feature::*;
use super::vector::PlaneVector;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Point: ({}, {})>", self.x, self.y)
    }
}

impl Inclusion<Line> for Point {
    fn is_included(&self, cpt: &Line) -> bool {
        is_in(self, cpt)
    }
}

impl Superposition<Point> for Point {
    fn is_superposition(&self, cpt: &Point) -> bool {
        point_is_superposition(self, cpt)
    }
}

impl CalcDistance<Point, f64> for Point {
    fn calc_d(&self, cpt: &Point) -> f64 {
        calc_point_d(self, cpt)
    }
}

impl CalcDistance<Line, f64> for Point {
    fn calc_d(&self, cpt: &Line) -> f64 {
        calc_point_line_d(self, cpt)
    }
}


/// function sample:
/// 
///     k1 * x + k2 * y = b
pub struct Line {
    fn_args: (f64, f64, f64)
}


impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Result<Line, ()> {
        match calc_line_func(p1, p2) {
            Ok(func_args) => {Ok(Line{fn_args: func_args})}
            Err(()) => {Err(())}
        }
    }
    
    /// k1 * x + k2 * y = b
    pub fn from(k1: f64, k2: f64, b: f64) -> Result<Line, ()> {
        if k1 == 0.0 && k2 == 0.0{
            Err(())
        } else {
            Ok(Self{fn_args: (k1, k2, b)})
        }
    } 

    pub fn func_args(&self) -> (f64, f64, f64) {
        self.fn_args
    }

    pub fn get_direction_vec(&self) -> PlaneVector {
        PlaneVector(self.fn_args.0, -self.fn_args.1)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (k1, k2, b) = self.func_args();
        if k1 == 0.0 {
            write!(f, "<Line {{(x, y)|{k2}y={b}}}>")
        } else if k2 == 0.0 {
            write!(f, "<Line {{(x, y)|{k1}x={b}}}>")
        } else {
            if k2 > 0.0 {
                write!(f, "<Line {{(x, y)|{k1}x+{k2}y={b}}}>")
            } else { // k2 < 0.0
                write!(f, "<Line {{(x, y)|{k1}x{k2}y={b}}}>")
            }
        }
    }
}

impl Inclusion<Point> for Line {
    fn is_included(&self, cpt: &Point) -> bool {
        is_in(cpt, self)
    }
}

impl Parallelism<Line> for Line {
    fn is_parallel(&self, cpt: &Line) -> bool {
        is_parallel(self, cpt)
    }   
}

impl Vertical<Line> for Line {
    fn is_vertical(&self, cpt: &Line) -> bool {
        is_vertical(self, cpt)
    }
}

impl Superposition<Line> for Line {
    fn is_superposition(&self, cpt: &Line) -> bool {
        line_is_superposition(self, cpt)
    }
}

impl CalcDistance<Point, f64> for Line {
    fn calc_d(&self, cpt: &Point) -> f64 {
        calc_point_line_d(cpt, self)
    }
}

impl CalcDistance<Line, Result<f64, ()>> for Line {
    fn calc_d(&self, cpt: &Line) -> Result<f64, ()> {
        calc_line_d(self, cpt)
    }
}

impl CalcAngle<Line> for Line {
    fn calc_angle(&self, cpt: &Line) -> f64 {
        calc_angle(self, cpt)
    }
}

impl CalcIntersection<Line> for Line {
    fn calc_intersection(&self, cpt: &Line) -> Result<Point, ()> {
        calc_intersection(self, cpt)
    }
}
