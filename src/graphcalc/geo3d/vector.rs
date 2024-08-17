use std::{fmt, ops};

use super::component::{Line, Plane, Point};

pub struct SpaceVector{x: f64, y: f64, z: f64}

impl SpaceVector {
    pub fn new(x: f64, y: f64 ,z: f64) -> Self{
        SpaceVector{x, y, z}
    }
    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn copy(&self) -> Self {
        SpaceVector{x: self.x, y: self.y, z: self.z}
    }
    pub fn to_line(&self, p: &Point) -> Result<Line, ()> {
        let (x_p, y_p, z_p) = p.pos();
        match Line::new(self.x, -self.y, self.x*y_p - self.y*x_p,
                        self.x, -self.z, self.x*z_p - self.z*x_p) {
            Ok(ln) => {Ok(ln)}
            Err(()) => {Err(())}
        }
    }
    pub fn to_plane(&self, p: &Point) -> Result<Plane, ()> {
        let (x_p, y_p, z_p) = p.pos();
        match Plane::new(self.x, self.y, self.z, self.x*x_p+self.y*y_p+self.z*z_p) {
            Ok(pn) => {Ok(pn)}
            Err(()) => {Err(())}
        }
    }
}

impl fmt::Display for SpaceVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Vector ({}, {}, {})>", self.x, self.y, self.z)
    }
}

impl ops::Add for &SpaceVector {
    type Output = SpaceVector;
    fn add(self, other: Self) -> Self::Output {
        SpaceVector{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl ops::Sub for &SpaceVector {
    type Output = SpaceVector;
    fn sub(self, other: Self) -> Self::Output {
        SpaceVector{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl ops::Neg for &SpaceVector {
    type Output = SpaceVector;
    fn neg(self) -> Self::Output {
        SpaceVector{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::Mul<f64> for &SpaceVector {
    type Output = SpaceVector;
    fn mul(self, other: f64) -> Self::Output {
        SpaceVector{x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

/// inner product of vector
impl ops::Mul<&SpaceVector> for &SpaceVector {
    type Output = f64;
    fn mul<'a>(self, other: &'a SpaceVector) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ops::Div<f64> for &SpaceVector {
    type Output = SpaceVector;
    fn div(self, other: f64) -> Self::Output {
        SpaceVector{x: self.x / other, y: self.y / other, z: self.z / other}
    }
}

/// outer product of vector
impl ops::Rem for &SpaceVector {
    type Output = SpaceVector;
    fn rem(self, other: Self) -> Self::Output {
        SpaceVector{
            x: self.y*other.z-self.z*other.y,
            y: -self.x*other.z+self.z*other.x,
            z: self.x*other.y-self.y*other.x,
        }
    }
}
