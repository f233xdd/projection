use std::fmt;

use super::{
    tool::*,
    super::{
        algebra::vector::PlaneVec,
        geo_err,
        GeoResult,
        interface,
    },
};

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }
    pub fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x; self.y = y;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Point: ({}, {})>", self.x, self.y)
    }
}

impl interface::Contain<Line> for Point {
    fn is_included(&self, cpt: &Line) -> bool {
        is_in(self, cpt)
    }
}

impl interface::Superposition<Point> for Point {
    fn is_superposition(&self, cpt: &Point) -> bool {
        point_is_superposition(self, cpt)
    }
}

impl interface::CalcDistance<Point, f64> for Point {
    fn calc_d(&self, cpt: &Point) -> f64 {
        calc_point_d(self, cpt)
    }
}

impl interface::CalcDistance<Line, f64> for Point {
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
    pub fn new(k1: f64, k2: f64, b: f64) -> Result<Self, geo_err::InvalidFnArgError> {
        if k1 == 0.0 && k2 == 0.0 {
            Err(geo_err::InvalidFnArgError())
        } else {
            Ok(Self {fn_args: (k1, k2, b)})
        }
    } 
    pub fn from(p1: &Point, p2: &Point) -> Result<Self, geo_err::SuperpositionError> {
        match calc_line_fn(p1, p2) {
            Ok(func_args) => Ok(Self {fn_args: func_args}),
            Err(e) => Err(e)
        }
    }
    pub fn fn_args(&self) -> (f64, f64, f64) {
        self.fn_args
    }
    pub fn get_direction_vec(&self) -> PlaneVec {
        PlaneVec::new([self.fn_args.1, -self.fn_args.0])
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

impl interface::Contain<Point> for Line {
    fn is_included(&self, cpt: &Point) -> bool {
        is_in(cpt, self)
    }
}
impl interface::Parallel<Line> for Line {
    fn is_parallel(&self, cpt: &Line) -> bool {
        is_parallel(self, cpt)
    }   
}
impl interface::Vertical<Line> for Line {
    fn is_vertical(&self, cpt: &Line) -> bool {
        is_vertical(self, cpt)
    }
}
impl interface::Superposition<Line> for Line {
    fn is_superposition(&self, cpt: &Line) -> bool {
        line_is_superposition(self, cpt)
    }
}
impl interface::CalcDistance<Point, f64> for Line {
    fn calc_d(&self, cpt: &Point) -> f64 {
        calc_point_line_d(cpt, self)
    }
}
impl interface::CalcDistance<Line, GeoResult<f64>> for Line {
    fn calc_d(&self, cpt: &Line) -> GeoResult<f64> {
        Ok(calc_line_d(self, cpt)?)
    }
}

impl interface::CalcAngle<Line> for Line {
    fn calc_angle(&self, cpt: &Line) -> f64 {
        calc_angle(self, cpt)
    }
}

impl interface::CalcIntersection<Line, GeoResult<Point>> for Line {
    fn calc_intersection(&self, cpt: &Line) -> GeoResult<Point> {
        Ok(calc_intersection(self, cpt)?)
    }
}

pub enum Geo2DComponent {
    Point(Point),
    Line(Line),
}

impl From<Point> for Geo2DComponent {
    fn from(value: Point) -> Self {
        Self::Point(value)
    }
}
impl From<Line> for Geo2DComponent {
    fn from(value: Line) -> Self {
        Self::Line(value)
    }
}
