use super::super::{geo2d, geo3d, geo_err};
use super::calc::approximate;

pub struct Vector<const L: usize> {
    data: [f64; L],
}

impl<const L: usize> Vector<L> {
    pub fn new(data: [f64; L]) -> Self {
        Self { data }
    }
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        Self {
            data: self
                .data
                .iter()
                .map(|x| x / norm)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

pub type PlaneVec = Vector<2>;
pub type SpaceVec = Vector<3>;

impl PlaneVec {
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn to_line(&self, p: &geo2d::Point) -> Result<geo2d::Line, geo_err::InvalidFnArgError> {
        let (x_p, y_p) = p.pos();
        geo2d::Line::new(-self.y(), self.x(), self.x() * y_p - self.y() * x_p)
    }
    pub fn cross(&self, rhs: &Self) -> SpaceVec {
        SpaceVec {
            data: [0.0, 0.0, self.x() * rhs.y() - self.y() * rhs.x()],
        }
    }
}

impl SpaceVec {
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn z(&self) -> f64 {
        self.data[2]
    }
    pub fn to_line(&self, p: &geo3d::Point) -> Result<geo3d::Line, geo_err::InvalidFnArgError> {
        let (x_p, y_p, z_p) = p.pos();
        if !approximate(self.x(), 0.0) {
            geo3d::Line::new(
                self.y(),
                -self.x(),
                0.0,
                self.y() * x_p - self.x() * y_p,
                self.z(),
                0.0,
                -self.x(),
                self.z() * x_p - self.x() * z_p,
            )
        } else {
            geo3d::Line::new(
                0.0,
                self.z(),
                -self.y(),
                self.z() * y_p - self.y() * z_p,
                1.0,
                0.0,
                0.0,
                x_p,
            )
        }
    }
    pub fn to_plane(&self, p: &geo3d::Point) -> Result<geo3d::Plane, geo_err::InvalidFnArgError> {
        let (x_p, y_p, z_p) = p.pos();
        match geo3d::Plane::new(
            self.x(),
            self.y(),
            self.z(),
            self.x() * x_p + self.y() * y_p + self.z() * z_p,
        ) {
            Ok(pn) => Ok(pn),
            Err(e) => Err(e),
        }
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        SpaceVec {
            data: [
                self.y() * rhs.z() - self.z() * rhs.y(),
                -self.x() * rhs.z() + self.z() * rhs.x(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            ],
        }
    }
}

impl<const L: usize> std::ops::Add for Vector<L> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Add for &Vector<L> {
    type Output = Vector<L>;

    fn add(self, other: Self) -> Vector<L> {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Sub for Vector<L> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Sub for &Vector<L> {
    type Output = Vector<L>;

    fn sub(self, other: Self) -> Vector<L> {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Mul<f64> for Vector<L> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|x| x * scalar)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Mul<f64> for &Vector<L> {
    type Output = Vector<L>;

    fn mul(self, scalar: f64) -> Vector<L> {
        Vector {
            data: self
                .data
                .iter()
                .map(|x| x * scalar)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Mul<Vector<L>> for Vector<L> {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<const L: usize> std::ops::Mul<&Vector<L>> for &Vector<L> {
    type Output = f64;

    fn mul(self, other: &Vector<L>) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<const L: usize> std::ops::Div<f64> for Vector<L> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|x| x / scalar)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Div<f64> for &Vector<L> {
    type Output = Vector<L>;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            data: self
                .data
                .iter()
                .map(|x| x / rhs)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> PartialEq for Vector<L> {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(a, b)| approximate(*a, *b))
    }
}

impl<const L: usize> std::fmt::Display for Vector<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, x) in self.data.iter().enumerate() {
            write!(f, "{}", x)?;
            if i < L - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = Vector::new([1.0, 2.0, 3.0]);
    let v2 = Vector::new([4.0, 5.0, 6.0]);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 + v2 = {}", &v1 + &v2);
    println!("v1 - v2 = {}", &v1 - &v2);
    println!("v1 * 2 = {}", &v1 * 2.0);
    println!("v1 / 2 = {}", &v1 / 2.0);
    println!("v1 * v2 = {}", &v1 * &v2);
    println!("v1 norm = {}", v1.norm());
    println!("v1 normalized = {}", v1.normalize());

    Ok(())
}
