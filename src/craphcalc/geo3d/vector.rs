use std::{fmt, ops};

use super::component::{Line, Plane, Point};

pub struct SpaceVector(pub f64, pub f64, pub f64);

impl SpaceVector {
    pub fn new(&self, x: f64, y: f64 ,z: f64) -> Self{
        Self(x, y, z)
    }
    pub fn len(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    pub fn copy(&self) -> Self {
        SpaceVector(self.0, self.1, self.2)
    }
    pub fn to_line(&self, p: &Point) -> Result<Line, ()> {
        match Line::from(self.0, -self.1, self.0*p.y - self.1*p.x,
                        self.0, -self.2, self.0*p.z - self.2*p.x) {
            Ok(ln) => {Ok(ln)}
            Err(()) => {Err(())}
        }
    }
    pub fn to_plane(&self, p: &Point) -> Result<Plane, ()> {
        match Plane::from(self.0, self.1, self.2, self.0*p.x+self.1*p.y+self.2*p.z) {
            Ok(pn) => {Ok(pn)}
            Err(()) => {Err(())}
        }
    }
}

impl fmt::Display for SpaceVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl ops::Add for SpaceVector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        SpaceVector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Sub for SpaceVector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        SpaceVector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::Neg for SpaceVector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        SpaceVector(-self.0, -self.1, -self.2)
    }
}

impl ops::Mul<f64> for SpaceVector {
    type Output = SpaceVector;
    fn mul(self, other: f64) -> Self::Output {
        SpaceVector(self.0 * other, self.1 * other, self.2 * other)
    }
}

/// inner product of vector
impl ops::Mul<SpaceVector> for SpaceVector {
    type Output = f64;
    fn mul(self, other: Self) -> Self::Output {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl ops::Div<f64> for SpaceVector {
    type Output = SpaceVector;
    fn div(self, other: f64) -> Self::Output {
        SpaceVector(self.0 / other, self.1 / other, self.2 / other)
    }
}

/// outer product of vector
impl ops::Rem for SpaceVector {
    type Output = SpaceVector;
    fn rem(self, other: Self) -> Self::Output {
        SpaceVector(self.1*other.2-self.2*other.1, -self.0*other.2+self.2*other.0, self.0*other.1-self.1*other.0)
    }
}
