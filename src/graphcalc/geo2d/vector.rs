use std::{ops, fmt};

use super::err;
use super::component::{Point, Line};
use super::super::geo3d::SpaceVector;

pub struct PlaneVector{x: f64, y: f64}

impl PlaneVector {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }
    pub fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    pub fn len(&self) -> f64  {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn copy(&self) -> Self {
        Self {x: self.x, y: self.y}
    }
    pub fn to_line(&self, p: &Point) -> Result<Line, err::InvalidFnArgError> {
        let (x_p, y_p) = p.pos();
        Line::new(-self.y, self.x, self.x * y_p - self.y * x_p)
    }
}

impl fmt::Display for PlaneVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Vector ({}, {})>", self.x, self.y)
    }
}

impl ops::Add for &PlaneVector {
    type Output = PlaneVector;
    fn add(self, other: Self) -> Self::Output {
        PlaneVector {x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Add for PlaneVector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Sub for &PlaneVector {
    type Output = PlaneVector;
    fn sub(self, other: Self) -> Self::Output {
        PlaneVector {x: self.x - other.x, y: self.y - other.y}
    }
}

impl ops::Sub for PlaneVector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl ops::Neg for &PlaneVector {
    type Output = PlaneVector;
    fn neg(self) -> Self::Output {
        PlaneVector {x: -self.x, y: -self.y}
    }
}

impl ops::Neg for PlaneVector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y}
    }
}

impl ops::Mul<f64> for &PlaneVector {
    type Output = PlaneVector;
    fn mul(self, other: f64) -> Self::Output {
        PlaneVector {x: self.x * other, y: self.y * other}
    }
}

impl ops::Mul<f64> for PlaneVector {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {x: self.x * other, y: self.y * other}
    }
}


/// inner product of vector
impl ops::Mul<Self> for &PlaneVector {
    type Output = f64;
    fn mul<'a>(self, other: &'a PlaneVector) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Mul<Self> for PlaneVector {
    type Output = f64;
    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Div<f64> for &PlaneVector {
    type Output = PlaneVector;
    fn div(self, other: f64) -> Self::Output {
        PlaneVector {x: self.x / other, y: self.y / other}
    }
}

impl ops::Div<f64> for PlaneVector {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {x: self.x / other, y: self.y / other}
    }
}

/// outer product of vector
impl ops::Rem for &PlaneVector {
    type Output = SpaceVector;
    fn rem(self, other: Self) -> Self::Output {
        SpaceVector::new(0.0, 0.0, self.x * other.y - self.y * other.x)
    }
}

impl ops::Rem for PlaneVector {
    type Output = SpaceVector;
    fn rem(self, other: Self) -> Self::Output {
        SpaceVector::new(0.0, 0.0, self.x * other.y - self.y * other.x)
    }
}

impl PartialEq<Self> for PlaneVector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
