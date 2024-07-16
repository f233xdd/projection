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
///     k1 * y + k2 * x = b
pub struct Line {
    func_args: [f64;3]
}


impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Result<Line, ()> {
        match calc_line_func(p1, p2) {
            Ok(func_args) => {Ok(Line{func_args})}
            Err(()) => {Err(())}
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

    pub fn get_func_args(&self) -> [f64; 3] {
        self.func_args
    }

    pub fn get_direction_vec(&self) -> PlaneVector {
        PlaneVector(self.func_args[0], -self.func_args[1])
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
