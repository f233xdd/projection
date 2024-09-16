use std::fmt;

use crate::graphcalc::algebra::calc::approximate;

use super::tool::*;
use super::tool::feature::*;
use super::vector::SpaceVector;

// 3D part
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }
    pub fn pos(&self) -> (f64, f64, f64) {
        (self.x, self.y ,self.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Point ({}, {}, {})>", self.x, self.y, self.z)
    }
}

impl Inclusion<Line> for Point {
    fn is_included(&self, cpt: &Line) -> bool {
        point_is_in_line(self, cpt)
    }
}

impl Inclusion<Plane> for Point {
    fn is_included(&self, cpt: &Plane) -> bool {
        point_is_in_plane(self, cpt)
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

impl CalcDistance<Plane, f64> for Point {
    fn calc_d(&self, cpt: &Plane) -> f64 {
        calc_point_plane_d(self, cpt)
    }
}

///  k11 * x + k12 * y + k13 * z = b1,
///  k21 * x + k22 * y + k23 * z = b2
pub struct Line {
    fn_args: ((f64, f64, f64, f64), (f64, f64, f64, f64))
}
impl Line {
    pub fn new(k11: f64, k12: f64, k13: f64, b1: f64,
                k21: f64, k22: f64, k23: f64, b2: f64) -> Result<Self, ()> {
        if !approximate(k11*k22, k12*k21) || !approximate(k11*k23, k13*k21) {
            Ok(Self {fn_args: ((k11, k12, k13, b1), (k21, k22, k23, b2))})
        } else {
            Err(())
        }
    }

    pub fn from(p1: &Point, p2: &Point) -> Result<Self, ()> {
        match calc_line_fn(p1, p2) {
            Ok(func_args) => {Ok(Self {fn_args: func_args})}
            Err(()) => {Err(())}
        }
    }

    pub fn fn_args(&self) -> ((f64, f64, f64, f64), (f64, f64, f64, f64)) {
        self.fn_args
    }

    pub fn get_direction_vec(&self) -> SpaceVector {
        let ((k11, k12, k13, _), (k21, k22, k23, _)) = self.fn_args();
        SpaceVector::new(k13*k22-k12*k23, k11*k23-k13*k21, k12*k21-k11*k22)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((k11, k12, k13, b1), (k21, k22, k23, b2)) = self.fn_args;
        let mut add_comma = true;
        let mut no_front_var: bool;

        write!(f, "<Line {{(x, y)|").unwrap();
        for (k1, k2, k3, b) in [(k11, k12, k13, b1), (k21, k22, k23, b2)] {
            if approximate(k1, 1.0) {
                write!(f, "x").unwrap();
                no_front_var = false;
            } else if approximate(k1, -1.0) {
                write!(f, "-x").unwrap();
                no_front_var = false;
            } else if approximate(k1, 0.0) {
                no_front_var = true;
            } else {
                write!(f, "{k1}x").unwrap();
                no_front_var = false;
            }

            if approximate(k2, 1.0) {
                if no_front_var {
                    write!(f, "y").unwrap();
                } else {
                    write!(f, "+y").unwrap();
                }
                no_front_var = false;
            } else if approximate(k2, -1.0) {
                write!(f, "-y").unwrap();
                no_front_var = false;
            } else if k2 > 0.0 {
                if no_front_var {
                    write!(f, "{k2}y").unwrap();
                } else {
                    write!(f, "+{k2}y").unwrap();
                }
                no_front_var = false;
            } else if k2 < 0.0 {
                write!(f, "{k2}y").unwrap();
                no_front_var = false;
            } else {}

            if approximate(k3, 1.0) {
                if no_front_var {
                    write!(f, "z").unwrap();
                } else {
                    write!(f, "+z").unwrap();
                }
            } else if approximate(k3, -1.0) {
                write!(f, "-z").unwrap();
            } else if k3 > 0.0 {
                if no_front_var {
                    write!(f, "{k3}z").unwrap();
                } else {
                    write!(f, "+{k3}z").unwrap();
                }
            } else if k3 < 0.0 {
                write!(f, "{k3}z").unwrap();
            } else {}
            write!(f, "={b}").unwrap();

            if add_comma {
                write!(f, ", ").unwrap();
                add_comma = false;
            } else {}
        }
        write!(f, "}}>")
    }
}

impl Inclusion<Plane> for Line {
    fn is_included(&self, cpt: &Plane) -> bool {
        line_is_in_plane(self, cpt)
    }
}

impl Parallelism<Line> for Line {
    fn is_parallel(&self, cpt: &Line) -> bool {
        line_is_parallel(self, cpt)
    }
}

impl Parallelism<Plane> for Line {
    fn is_parallel(&self, cpt: &Plane) -> bool {
        line_plane_is_parallel(self, cpt)
    }
}

impl Vertical<Line> for Line {
    fn is_vertical(&self, cpt: &Line) -> bool {
        line_is_vertical(self, cpt)
    }
}

impl Vertical<Plane> for Line {
    fn is_vertical(&self, cpt: &Plane) -> bool {
        line_plane_is_vertical(self, cpt)
    }
}

impl Coplanarity<Line> for Line {
    fn is_coplanar(&self, cpt: &Line) -> bool {
        line_is_coplanar(self, cpt)
    }
}

impl Superposition<Line> for Line {
    fn is_superposition(&self, cpt: &Line) -> bool {
        line_is_superposition(self, cpt)
    }
}

impl CalcDistance<Point, f64> for Line{
    fn calc_d(&self, cpt: &Point) -> f64 {
        calc_point_line_d(cpt, self)
    }
}

impl CalcDistance<Line, f64> for Line{
    fn calc_d(&self, cpt: &Line) -> f64 {
        calc_line_d(self, cpt)
    }
}

impl CalcDistance<Plane, Result<f64, ()>> for Line{
    fn calc_d(&self, cpt: &Plane) -> Result<f64, ()> {
        calc_line_plane_d(self, cpt)
    }
}

impl CalcAngle<Line> for Line {
    fn calc_angle(&self, cpt: &Line) -> f64 {
        calc_line_angle(self, cpt)
    }
}

impl CalcAngle<Plane> for Line {
    fn calc_angle(&self, cpt: &Plane) -> f64 {
        calc_line_plane_angle(self, cpt)
    }
}

impl CalcIntersection<Line, Result<Point, ()>> for Line {
    fn calc_intersection(&self, cpt: &Line) -> Result<Point, ()> {
        calc_line_intersection(self, cpt)
    }
}

impl CalcIntersection<Plane, Result<Point, ()>> for Line {
    fn calc_intersection(&self, cpt: &Plane) -> Result<Point, ()> {
        calc_line_plane_intersection(self, cpt)
    }
}

/// k1 * x + k2 * y + k3 * z = b
pub struct Plane {
    func_args: (f64, f64, f64, f64)
}

impl Plane {
    /// k1 * x + k2 * y + k3 * z = b
    pub fn new(k1: f64, k2: f64, k3: f64, b: f64) -> Result<Self, ()> {
        if k1 == 0.0 && k2 == 0.0 && k3 == 0.0 {
            Err(())
        } else {
            Ok(Self {func_args: (k1, k2, k3, b)})
        }
    }

    pub fn from(p1: &Point, p2: &Point, p3: &Point) -> Result<Self, ()>  {
        match calc_plane_fn(p1, p2, p3) {
            Ok(fn_args) => {Ok(Self {func_args: fn_args})}
            Err(()) => {Err(())}
        }
    }

    pub fn fn_args(&self) -> (f64, f64, f64, f64) {
        self.func_args
    }

    pub fn get_normal_vec(&self) -> SpaceVector {
        SpaceVector::new(self.func_args.0, self.func_args.1, self.func_args.2)
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (k1, k2, k3, b) = self.fn_args();
        if k1 == 0.0 {
            write!(f, "<Plane {{(x, y, z)|").unwrap();
        } else if k1 == 1.0 {
            write!(f, "<Plane {{(x, y, z)|x").unwrap();
        } else if k1 == -1.0 {
            write!(f, "<Plane {{(x, y, z)|-x").unwrap();
        } else {
            write!(f, "<Plane {{(x, y, z)|{k1}x").unwrap();
        }

        if k2 > 0.0 {
            if k2 == 1.0 {
                write!(f, "+y").unwrap();
            } else {
                write!(f, "+{k2}y").unwrap();
            }
        } else if k2 < 0.0 {
            if k2 == -1.0 {
                write!(f, "-y").unwrap();
            } else {
                write!(f, "{k2}y").unwrap();
            }
        } else {}

        if k3 > 0.0 {
            if k3 == 1.0 {
                write!(f, "+z={b}}}>")
            } else {
                write!(f, "+{k3}z={b}}}>")
            }
        } else if k3 < 0.0 {
            if k3 == -1.0 {
                write!(f, "-z={b}}}>")
            } else {
                write!(f, "{k3}z={b}}}>")
            }
        } else {
            write!(f, "={b}}}>")
        }
    }
}

impl Parallelism<Line> for Plane {
    fn is_parallel(&self, cpt: &Line) -> bool {
        line_plane_is_parallel(cpt, self)
    }
}

impl Parallelism<Plane> for Plane {
    fn is_parallel(&self, cpt: &Plane) -> bool {
        plane_is_parallel(self, cpt)
    }
}

impl Vertical<Line> for Plane {
    fn is_vertical(&self, cpt: &Line) -> bool {
        line_plane_is_vertical(cpt, self)
    }
}

impl Vertical<Plane> for Plane {
    fn is_vertical(&self, cpt: &Plane) -> bool {
        plane_is_vertical(self, cpt)
    }
}

impl Superposition<Plane> for Plane {
    fn is_superposition(&self, cpt: &Plane) -> bool {
        plane_is_superposition(self, cpt)
    }
}

impl CalcDistance<Point, f64> for Plane {
    fn calc_d(&self, cpt: &Point) -> f64 {
        calc_point_plane_d(cpt, self)
    }
}

impl CalcDistance<Line, Result<f64, ()>> for Plane {
    fn calc_d(&self, cpt: &Line) -> Result<f64, ()> {
        calc_line_plane_d(cpt, self)
    }
}

impl CalcDistance<Plane, Result<f64, ()>> for Plane {
    fn calc_d(&self, cpt: &Plane) -> Result<f64, ()> {
        calc_plane_d(self, cpt)
    }
}

impl CalcAngle<Line> for Plane {
    fn calc_angle(&self, cpt: &Line) -> f64 {
        calc_line_plane_angle(cpt, self)
    }
}

impl CalcAngle<Plane> for Plane {
    fn calc_angle(&self, cpt: &Plane) -> f64 {
        calc_plane_angle(self, cpt)
    }
}

impl CalcIntersection<Line, Result<Point, ()>> for Plane {
    fn calc_intersection(&self, cpt: &Line) -> Result<Point, ()> {
        calc_line_plane_intersection(cpt, &self)
    }
}

impl CalcIntersection<Plane, Result<Line, ()>> for Plane {
    fn calc_intersection(&self, cpt: &Plane) -> Result<Line, ()> {
        calc_plane_intersection(self, cpt)
    }
}