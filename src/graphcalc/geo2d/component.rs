use std::fmt;

// 2D part
use super::tool::*;
use super::tool::feature::*;
use super::vector::PlaneVector;
use super::err;

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
    pub fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
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



/// k1 * x + k2 * y = b
pub struct Line {
    fn_args: (f64, f64, f64)
}


impl Line {
    /// k1 * x + k2 * y = b
    pub fn new(k1: f64, k2: f64, b: f64) -> Result<Line, err::InvalidFnArgError> {
        if k1 == 0.0 && k2 == 0.0{
            Err(err::InvalidFnArgError())
        } else {
            Ok(Self{fn_args: (k1, k2, b)})
        }
    } 

    pub fn from(p1: &Point, p2: &Point) -> Result<Line, err::InterpositionError> {
        match calc_line_fn(p1, p2) {
            Ok(func_args) => {Ok(Line{fn_args: func_args})}
            Err(e) => {Err(e)}
        }
    }

    pub fn fn_args(&self) -> (f64, f64, f64) {
        self.fn_args
    }

    pub fn get_direction_vec(&self) -> PlaneVector {
        PlaneVector::new(self.fn_args.1, -self.fn_args.0)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (k1, k2, b) = self.fn_args;
        write!(f, "<Line {{(x, y)|").unwrap();

        if k1 == 1.0 {
            write!(f, "x").unwrap();
        } else if k1 == -1.0 {
            write!(f, "-x").unwrap();
        } else if k1 == 0.0 {} else {
            write!(f, "{k1}x").unwrap();
        }

        if k2 == 1.0 {
            write!(f, "+y={b}}}>")
        } else if k2 == -1.0 {
            write!(f, "-y={b}}}>")
        } else if k2 > 0.0 {
            write!(f, "+{k2}y={b}}}>")
        } else if k2 < 0.0 {
            write!(f, "{k2}y={b}}}>")
        } else {
            write!(f, "={b}}}>")
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

impl CalcDistance<Line, Result<f64, err::NotParallelError>> for Line {
    fn calc_d(&self, cpt: &Line) -> Result<f64, err::NotParallelError> {
        calc_line_d(self, cpt)
    }
}

impl CalcAngle<Line> for Line {
    fn calc_angle(&self, cpt: &Line) -> f64 {
        calc_angle(self, cpt)
    }
}

impl CalcIntersection<Line> for Line {
    fn calc_intersection(&self, cpt: &Line) -> Result<Point, err::ParallelError> {
        calc_intersection(self, cpt)
    }
}
