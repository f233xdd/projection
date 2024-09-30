use std::fmt;

// 2D part
use super::tool::*;
use super::vector::PlaneVector;
use super::err;

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
    pub fn is_superposition(&self, cpt2d: &Geo2DComponent) -> Result<bool, err::MismatchedComponentError> {
        match cpt2d {
            Geo2DComponent::Point(p) => Ok(point_is_superposition(self, p)),
            _ => Err(err::MismatchedComponentError())
        }
    }
    pub fn is_contained(&self, cpt2d: &Geo2DComponent) -> Result<bool, err::MismatchedComponentError> {
        match cpt2d {
            Geo2DComponent::Line(ln) => Ok(is_in(self, ln)),
            _ => Err(err::MismatchedComponentError())
        }
    }
    pub fn calc_d(&self, cpt2d: &Geo2DComponent) -> f64 {
        match cpt2d {
            Geo2DComponent::Point(p) => calc_point_d(self, p),
            Geo2DComponent::Line(ln) => calc_point_line_d(self, ln)
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Point: ({}, {})>", self.x, self.y)
    }
}

/// k1 * x + k2 * y = b
pub struct Line {
    fn_args: (f64, f64, f64)
}


impl Line {
    /// k1 * x + k2 * y = b
    pub fn new(k1: f64, k2: f64, b: f64) -> Result<Self, err::InvalidFnArgError> {
        if k1 == 0.0 && k2 == 0.0 {
            Err(err::InvalidFnArgError())
        } else {
            Ok(Self {fn_args: (k1, k2, b)})
        }
    } 
    pub fn from(p1: &Point, p2: &Point) -> Result<Self, err::SuperpositionError> {
        match calc_line_fn(p1, p2) {
            Ok(func_args) => {Ok(Self {fn_args: func_args})}
            Err(e) => {Err(e)}
        }
    }
    pub fn fn_args(&self) -> (f64, f64, f64) {
        self.fn_args
    }
    pub fn get_direction_vec(&self) -> PlaneVector {
        PlaneVector::new(self.fn_args.1, -self.fn_args.0)
    }
    pub fn is_superposition(&self, cpt2d: &Geo2DComponent) -> Result<bool, err::MismatchedComponentError> {
        match cpt2d {
            Geo2DComponent::Line(ln) => Ok(line_is_superposition(self, ln)),
            _ => Err(err::MismatchedComponentError()),
        }
    }
    pub fn is_parallel(&self, cpt2d: &Geo2DComponent) -> Result<bool, err::MismatchedComponentError> {
        match cpt2d {
            Geo2DComponent::Line(ln) => Ok(is_parallel(self, ln)),
            _ => Err(err::MismatchedComponentError()),
        }
    }
    pub fn is_vertical(&self, cpt2d: &Geo2DComponent) -> Result<bool, err::MismatchedComponentError> {
        match cpt2d {
            Geo2DComponent::Line(ln) => Ok(is_vertical(self, ln)),
            _ => Err(err::MismatchedComponentError()),
        }
    }
    pub fn calc_d(&self, cpt2d: &Geo2DComponent) -> Result<f64, err::NotParallelError> {
        match cpt2d {
            Geo2DComponent::Point(p) => Ok(calc_point_line_d(p, self)),
            Geo2DComponent::Line(ln) => calc_line_d(self, ln),
        }
    }
    pub fn calc_angle(&self, cpt2d: &Geo2DComponent) -> Result<f64, err::MismatchedComponentError> {
        match cpt2d {
            Geo2DComponent::Line(ln) => Ok(calc_angle(ln, self)),
            _ => Err(err::MismatchedComponentError())
        }
    }
    pub fn calc_intersection(&self, cpt2d: &Geo2DComponent) -> Result<Point, err::Geo2DError> {
        match cpt2d {
            Geo2DComponent::Line(ln) => match calc_intersection(self, ln) {
                Ok(i) => Ok(i),
                Err(e) => Err(err::Geo2DError::from(err::PositionError::from(e)))
            },
            _ => Err(err::Geo2DError::from(err::MismatchedComponentError()))
        }
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
