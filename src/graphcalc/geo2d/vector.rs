use std::{ops, fmt};

use super::component::{Point, Line};
use super::super::geo3d::SpaceVector;

pub struct PlaneVector(pub f64, pub f64);

impl PlaneVector {
    pub fn new(x: f64, y: f64) -> Self  {
        PlaneVector(x, y)
    }
    pub fn len(&self) -> f64  {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
    pub fn copy(&self) -> Self {
        PlaneVector(self.0, self.1)
    }
    pub fn to_line(&self, p: &Point) -> Result<Line, ()> {
        Line::from(self.0, -self.1, self.0 * p.y - self.1 * p.x)
    }
}

impl fmt::Display for PlaneVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl ops::Add for &PlaneVector {
    type Output = PlaneVector;
    fn add(self, other: Self) -> Self::Output {
        PlaneVector(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::Sub for &PlaneVector {
    type Output = PlaneVector;
    fn sub(self, other: Self) -> Self::Output {
        PlaneVector(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::Neg for &PlaneVector {
    type Output = PlaneVector;
    fn neg(self) -> Self::Output {
        PlaneVector(-self.0, -self.1)
    }
}

impl ops::Mul<f64> for &PlaneVector {
    type Output = PlaneVector;
    fn mul(self, other: f64) -> Self::Output {
        PlaneVector(self.0 * other, self.1 * other)
    }
}

/// inner product of vector
impl ops::Mul<&PlaneVector> for &PlaneVector {
    type Output = f64;
    fn mul<'a>(self, other: &'a PlaneVector) -> Self::Output {
        self.0 * other.0 + self.1 * other.1
    }
}

impl ops::Div<f64> for &PlaneVector {
    type Output = PlaneVector;
    fn div(self, other: f64) -> Self::Output {
        PlaneVector(self.0 / other, self.1 / other)
    }
}

/// outer product of vector
impl ops::Rem for &PlaneVector {
    type Output = SpaceVector;
    fn rem(self, other: Self) -> Self::Output {
        SpaceVector(0.0, 0.0, self.0 * other.1 - self.1 * other.0)
    }
}
