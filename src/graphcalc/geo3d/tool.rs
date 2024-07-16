// 3D part
use super::{component::*, SpaceVector};

/// function sample:
/// 
///     | k11 * y + k12 * x = b1
///     | k21 * z + k22 * x = b2
pub fn calc_line_func(p1: &Point, p2: &Point) -> Result<[[f64; 3]; 2], ()> {
    if (p2.x != p1.x) || (p2.y != p1.y) || (p2.z != p1.z) {
        return Ok([[p2.x - p1.x, p1.y - p2.y, p2.x * p1.y - p1.x * p2.y], 
                    [p2.x - p1.x, p1.z - p2.z, p2.x * p1.z - p1.x * p2.z]]);
    } else {
        return Err(());
    }
}


/// k1 * x + k2 * y + k3 * z = b
pub fn calc_plane_func(p1: &Point, p2: &Point, p3: &Point) -> Result<[f64; 4], ()> {
    let k1 = (p1.x - p2.x) * (p2.y - p3.y) - (p1.y - p2.y) * (p2.x - p3.x);
    let k2 = (p1.y - p2.y) * (p2.z - p3.z) - (p1.z - p2.z) * (p2.y - p3.y);
    let k3 = (p1.z - p2.z) * (p2.x - p3.x) - (p1.x - p2.x) * (p2.z - p3.z);
    let b = p1.x * k2 + p1.y * k3 + p1.z * k1;
    if k1 != 0.0 && k2 != 0.0 && k3 != 0.0 {
        return Ok([k1, k2, k3, b]);
    } else {
        return Err(());
    }
}

pub fn vec_to_line(vec: SpaceVector, p: Point) -> Result<Line, ()> {
    vec.to_line(&p)
}

pub fn vec_to_plane(vec: SpaceVector, p: Point) -> Result<Plane, ()> {
    vec.to_plane(&p)
}

pub fn point_is_in_line(p: &Point, ln: &Line) -> bool {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    (k11 * p.y + k12 * p.x == b1) && (k21 * p.y + k22 * p.x == b2)
}

pub fn point_is_in_plane(p: &Point, pn: &Plane) -> bool {
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    k1 * p.x + k2 * p.y + k3 * p.z == b
}

pub fn line_is_in_plane(ln: &Line, pn: &Plane) -> bool {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    (k1 * k11 * k21 == k2 * k12 * k21 + k3 * k22 * k11) && 
    (b * k11 * k21 == b1 * k2 * k21 + b2 * k3 * k11)
}

pub fn line_is_parallel(ln1: &Line, ln2: &Line) -> bool {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let b11 = ln1.get_func_args()[0][2];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let b12 = ln1.get_func_args()[1][2];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let b21 = ln2.get_func_args()[0][2];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    let b22 = ln2.get_func_args()[1][2];
    (k111 * k212 == k112 * k211) && (k121 * k222 == k122 * k221) && 
    ((k111 * b21 != k211 * b11) || (k121 * b22 != k221 * b12))
}


pub fn line_plane_is_parallel(ln: &Line, pn: &Plane) -> bool {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    (k1 * k11 * k21 == k2 * k12 * k21 + k3 * k22 * k11) && 
    (b * k11 * k21 != b1 * k2 * k21 + b2 * k3 * k11)
}

pub fn plane_is_parallel(pn1: &Plane, pn2: &Plane) -> bool {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let b1 = pn1.get_func_args()[3];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    let b2 = pn2.get_func_args()[3];
    (k11 * k22 == k21 * k12) && (k11 * k23 == k21 * k13) && ((k11 * b2 != k21 * b1))
}

pub fn line_is_vertical(ln1: &Line, ln2: &Line) -> bool {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    k111 * k121 * k211 * k221 + k112 * k121 * k212 * k221 + k111 * k122 * k211 * k222 == 0.0
}

pub fn line_plane_is_vertical(ln: &Line, pn: &Plane) -> bool {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    (k1 * k12 + k2 * k11 == 0.0) && (k1 * k22 + k3 * k21 == 0.0)
}

pub fn plane_is_vertical(pn1: &Plane, pn2: &Plane) -> bool {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    k11 * k21 + k12 * k22 + k13 * k23 == 0.0
}

// pub fn line_is_intersected(ln1: &Line, ln2: &Line) -> bool {
//     let k111 = ln1.get_func_args()[0][0];
//     let k112 = ln1.get_func_args()[0][1];
//     let b11 = ln1.get_func_args()[0][2];
//     let k121 = ln1.get_func_args()[1][0];
//     let k122 = ln1.get_func_args()[1][1];
//     let b12 = ln1.get_func_args()[1][2];
//     let k211 = ln2.get_func_args()[0][0];
//     let k212 = ln2.get_func_args()[0][1];
//     let b21 = ln2.get_func_args()[0][2];
//     let k221 = ln2.get_func_args()[1][0];
//     let k222 = ln2.get_func_args()[1][1];
//     let b22 = ln2.get_func_args()[1][2];
//     let v1 = k112 * k211 - k111 * k212;
//     let v2 = k122 * k221 - k121 * k222;
//     if v1 != 0.0 && v2 != 0.0 {
//         b11 * k211 - b21 * k111 / v1 == b12 * k221 - b22 * k121 / v2
//     } else {false}
// }

/// line superposition is included
pub fn line_is_coplanar(ln1: &Line, ln2: &Line) -> bool {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let b11 = ln1.get_func_args()[0][2];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let b12 = ln1.get_func_args()[1][2];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let b21 = ln2.get_func_args()[0][2];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    let b22 = ln2.get_func_args()[1][2];
    (k121 * k222 - k122 * k221) * (k211 * b11 - k111 * b21) == (k111 * k212 - k112 * k211) * (k221 * b12 - k121 * b22)
}

pub fn point_is_superposition(p1: &Point, p2: &Point) -> bool {
    (p1.x == p2.x) && (p1.y == p2.y) && (p1.z == p2.z)
}

pub fn line_is_superposition(ln1: &Line, ln2: &Line) -> bool {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let b11 = ln1.get_func_args()[0][2];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let b12 = ln1.get_func_args()[1][2];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let b21 = ln2.get_func_args()[0][2];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    let b22 = ln2.get_func_args()[1][2];
    (k111 * b21 == k211 * b11) && (k112 * b22 == k212 * b12) && 
    (k121 * b22 == k221 * b12) && (k122 * b22 == k222 * b12)
}

pub fn plane_is_superposition(pn1: &Plane, pn2: &Plane) -> bool {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let b1 = pn1.get_func_args()[3];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    let b2 = pn2.get_func_args()[3];
    (k11 * b2 == k21 * b1) && (k12 * b2 == k22 * b1) && (k13 * b2 == k23 * b1)
}

pub fn calc_point_d(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
}

pub fn calc_point_line_d(p: &Point, ln: &Line) -> f64 {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    (((k11*k22*p.y-k12*k21*p.z-b1*k22+b2*k12).powi(2) + 
    ((k12*p.x+k11*p.y-b1)*k21).powi(2) + ((k22*p.x+k21*p.z-b2)*k11).powi(2))/
    ((k11*k12).powi(2)+(k12*k21).powi(2)+(k11*k22).powi(2))).sqrt()
}

pub fn calc_line_d(ln1: &Line, ln2: &Line) -> f64 {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let b11 = ln1.get_func_args()[0][2];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let b12 = ln1.get_func_args()[1][2];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let b21 = ln2.get_func_args()[0][2];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    let b22 = ln2.get_func_args()[1][2];
    let k1 = k112*k121*k211*k222-k212*k221*k111*k122;
    let k2 = k111*k211*(k121*k222-k221*k122);
    let k3 = k121*k221*(k112*k211-k212*k111);
    ((k211*b11-k111*b21)*(k121*k222-k221*k122)+(k221*b12-k121*b22)*(k112*k211-k212*k111)).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt()
}

pub fn calc_point_plane_d(p: &Point, pn: &Plane) -> f64 {
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    (k1*p.x+k2*p.y+k3*p.z-b).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt()
}

pub fn calc_line_plane_d(ln: &Line, pn: &Plane) -> Result<f64, ()> {
    let k11 = ln.get_func_args()[0][0];
    // let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    // let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    if line_plane_is_parallel(ln, pn) {
        Ok((k2*b1/k11+k3*b2/k21-b).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt())
    } else {Err(())}
}

pub fn calc_plane_d(pn1: &Plane, pn2: &Plane) -> Result<f64, ()> {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let b1 = pn1.get_func_args()[3];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    let b2 = pn2.get_func_args()[3];
    if plane_is_parallel(pn1, pn2) {
        let k = if k11 != 0.0 {k21/k11}
                    else if k12 != 0.0 {k22/k12}
                    else {k23/k13};
        Ok((k*b1-b2).abs()/(k21.powi(2)+k22.powi(2)+k23.powi(2)).sqrt())
    } else {Err(())}
}

pub fn calc_line_angle(ln1: &Line, ln2: &Line) -> f64 {
    let vec1 = ln1.get_direction_vec();
    let vec2 = ln2.get_direction_vec(); 
    ((&vec1 * &vec2).abs() / (vec1.len() * vec2.len())).acos()
}

pub fn calc_line_plane_angle(ln: &Line, pn: &Plane) -> f64 {
    let vec1 = ln.get_direction_vec();
    let vec2 = pn.get_normal_vec();
    ((&vec1 * &vec2).abs() / (vec1.len() * vec2.len())).asin()
}

pub fn calc_plane_angle(pn1: &Plane, pn2: &Plane) -> f64 {
    let vec1 = pn1.get_normal_vec();
    let vec2 = pn2.get_normal_vec();
    ((&vec1 * &vec2).abs() / (vec1.len() * vec2.len())).acos()
}

pub fn calc_line_intersection(ln1: &Line, ln2: &Line) -> Result<Point, ()> {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let b11 = ln1.get_func_args()[0][2];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let b12 = ln1.get_func_args()[1][2];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let b21 = ln2.get_func_args()[0][2];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    let b22 = ln2.get_func_args()[1][2];
    let x1 = (k211 * b11 - k111 * b21) / (k112 * k211 - k111 * k212);
    let x2 = (k221 * b12 - k121 * b22) / (k122 * k221 - k121 * k222);
    if x1 == x2 {
        let y = (k212 * b11 - k112 * b21) / (k111 * k212 - k112 * k211);
        let z = (k222 * b12 - k122 * b22) / (k121 * k222 - k122 * k221);
        Ok(Point{x: x1, y, z})
    } else {Err(())}
}
pub fn calc_line_plane_intersection(ln: &Line, pn: &Plane) -> Result<Point, ()> {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    let k = k1  * k11 * k21 - k2 * k12 * k21 - k3 * k11 * k22;
    if k != 0.0 {
        let x = (b * k11 * k21 - b1 * k2 * k21 - b2 * k3 * k11) / k;
        if k11 != 0.0 && k21 != 0.0 {
            let y = (b1 - k12 * x) / k11;
            let z = (b2 - k22 * x) / k21;
            Ok(Point{x, y, z})
        } else if k11 == 0.0 {
            let z = (b2 - k22 * x) / k21;
            let y = (b - k1 * x - k3 * z) / k2;
            Ok(Point{x, y, z})
        } else { // k21 == 0.0
            let y = (b1 - k12 * x) / k11;
            let z = (b - k1 * x - k2 * y) / k3;
            Ok(Point{x, y, z})
        }
    } else {
        Err(())
    }
}
pub fn calc_plane_intersection(pn1: &Plane, pn2: &Plane) -> Result<Line, ()> {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let b1 = pn1.get_func_args()[3];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    let b2 = pn2.get_func_args()[3];
    Line::from(k12*k23-k22*k13, k11*k23-k21*k13, b1*k23-b2*k13,
               k13*k22-k23*k12, k11*k22-k21*k12, b1*k22-b2*k12)
}

pub mod feature {
    pub trait Inclusion<T> {
        fn is_included(&self, cpt: &T) -> bool;
    }

    pub trait Parallelism<T> {
        fn is_parallel(&self, cpt: &T) -> bool;
    }

    pub trait Vertical<T> {
        fn is_vertical(&self, cpt: &T) -> bool;
    }

    pub trait Coplanarity<T> {
        fn is_coplanar(&self, cpt: &T) -> bool;
    }

    pub trait Superposition<T> {
        fn is_superposition(&self, cpt: &T) -> bool;
    }

    pub trait CalcDistance<T, U> {
        fn calc_d(&self, cpt: &T) -> U;
    }

    pub trait CalcAngle<T> {
        fn calc_angle(&self, cpt: &T) -> f64;
    }

    pub trait CalcIntersection<T, U> {
        fn calc_intersection(&self, cpt: &T) -> U;
    }
}