use super::tool::{calc_line_func, calc_plane_func};
use super::vector::SpaceVector;

// 3D part
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{x, y, z}
    }
}

/// function sample:
/// 
///     | k11 * y + k12 * x = b1
///     | k21 * z + k22 * x = b2
pub struct Line {
    func_args: [[f64; 3]; 2]
}


impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Result<Self, ()> {
        match calc_line_func(p1, p2) {
            Ok(func_args) => {Ok(Self{func_args})}
            Err(()) => {Err(())}
        }
    }
    
    pub fn from(k11: f64, k12: f64, b1: f64,
                k21: f64, k22: f64, b2: f64) -> Result<Self, ()> {
        if k11 == 0.0 && k12 == 0.0 {
            if b1 != 0.0 {
                return Err(());
            } else {
                if k21 == 0.0 && k22 == 0.0 {
                    return Err(());
                } else {
                    return Ok(Self{func_args: [[k11, k12, b1], [k21, k22, b2]]});
                }
            }
        } else if !(k11 == 0.0 && k21 == 0.0) {
            if k21 == 0.0 && k22 == 0.0 && b2 != 0.0 {
                return Err(());
            } else {
                return Ok(Self{func_args: [[k11, k12, b1], [k21, k22, b2]]});
            }
        } else  {
            return Err(());
        }
    }

    pub fn get_func_args(&self) -> [[f64; 3]; 2] {
        self.func_args
    }

    pub fn get_direction_vec(&self) -> SpaceVector { // TODO
        let k11 = self.func_args[0][0];
        let k12 = self.func_args[0][1];
        let k21 = self.func_args[1][0];
        let k22 = self.func_args[1][1];
        SpaceVector(-k11*k21, k12*k21, k11*k22)
    }
}


/// function sample:
/// 
///     k1 * x + k2 * y + k3 * z = b
pub struct Plane {
    func_args: [f64; 4]
}

impl Plane {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Result<Self, ()>  {
        match calc_plane_func(p1, p2, p3) {
            Ok(func_args) => {Ok(Self{func_args})}
            Err(()) => {Err(())}
        }
    }

    /// k1 * x + k2 * y + k3 * z = b
    pub fn from(k1: f64, k2: f64, k3: f64, b: f64) -> Result<Self, ()> {
        if k1 == 0.0 && k2 == 0.0 && k2 == 0.0{
            Err(())
        } else {
            Ok(Self{func_args: [k1, k2, k3, b]})
        }
    }

    pub fn get_func_args(&self) -> [f64; 4] {
        self.func_args
    }

    pub fn get_normal_vec(&self) -> SpaceVector {
        SpaceVector(self.func_args[0], self.func_args[1], self.func_args[2])
    }
}
