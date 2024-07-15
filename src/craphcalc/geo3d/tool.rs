use std::pin::Pin;

// 3D part
use super::{component::*, SpaceVector};

/// | k11 * y + k12 * x = b1
/// | k21 * z + k22 * x = b2
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

pub fn point_is_in_line(ln: &Line, p: &Point) -> bool {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let b1 = ln.get_func_args()[0][2];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let b2 = ln.get_func_args()[1][2];
    if (k11 * p.y + k12 * p.x == b1) && (k21 * p.y + k22 * p.x == b2) {true} else {false}
}

pub fn point_is_in_plane(pn: &Plane, p: &Point) -> bool {
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    let b = pn.get_func_args()[3];
    if k1 * p.x + k2 * p.y + k3 * p.z == b {true} else {false}
}

pub fn line_is_in_plane(pn: &Plane, ln: &Line) -> bool {
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
    if (k1 * k11 * k21 == k2 * k12 * k21 + k3 * k22 * k11) && 
    (b * k11 * k21 == b1 * k2 * k21 + b2 * k3 * k11) {true} else {false}
}

pub fn line_is_parallel(ln1: &Line, ln2: &Line) -> bool { // TODO: Superposition
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
    if (k111 * k212 == k112 * k211) && (k121 * k222 == k122 * k221) && 
    (k111 * b21 != k211 * b11) && (k121 * b22 == k221 * b12) {true} else {false}
}


pub fn line_plane_is_parallel(pn: &Plane, ln: &Line) -> bool {
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
    if (k1 * k11 * k21 == k2 * k12 * k21 + k3 * k22 * k11) && 
    (b * k11 * k21 != b1 * k2 * k21 + b2 * k3 * k11) {true} else {false}
}

pub fn plane_is_parallel(pn1: &Plane, pn2: &Plane) -> bool { // TODO: Superposition
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    if (k11 * k22 == k21 * k12) && (k11 * k23 == k21 * k13) {true} else {false}
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
    if k111 * k121 * k211 * k221 + k112 * k121 * k212 * k221 + k111 * k122 * k211 * k222 == 0.0 {true}
    else {false}
}

pub fn line_plane_is_vertical(pn: &Plane, ln: &Line) -> bool {
    let k11 = ln.get_func_args()[0][0];
    let k12 = ln.get_func_args()[0][1];
    let k21 = ln.get_func_args()[1][0];
    let k22 = ln.get_func_args()[1][1];
    let k1 = pn.get_func_args()[0];
    let k2 = pn.get_func_args()[1];
    let k3 = pn.get_func_args()[2];
    if (k1 * k12 + k2 * k11 == 0.0) && (k1 * k22 + k3 * k21 == 0.0) {true} else {false}
}

pub fn plane_is_vertical(pn1: &Plane, pn2: &Plane) -> bool {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    if k11 * k21 + k12 * k22 + k13 * k23 == 0.0 {true} else {false}
}

pub fn line_is_intersected(ln1: &Line, ln2: &Line) -> bool {
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
    let v1 = k112 * k211 - k111 * k212;
    let v2 = k122 * k221 - k121 * k222;
    if v1 != 0.0 && v2 != 0.0 {
        if b11 * k211 - b21 * k111 / v1 == b12 * k221 - b22 * k121 / v2 {true} else {false}
    } else {false}
}

/// line superposition is included
pub fn line_is_coplanar(ln1: &Line, ln2: &Line) -> bool {
    let k111 = ln1.get_func_args()[0][0];
    let k112 = ln1.get_func_args()[0][1];
    let k121 = ln1.get_func_args()[1][0];
    let k122 = ln1.get_func_args()[1][1];
    let k211 = ln2.get_func_args()[0][0];
    let k212 = ln2.get_func_args()[0][1];
    let k221 = ln2.get_func_args()[1][0];
    let k222 = ln2.get_func_args()[1][1];
    if (k111 * k212 == k112 * k211) && (k121 * k222 == k122 * k221) {true} else {false}
}

fn point_is_superposition(p1: &Point, p2: &Point) -> bool {
    if (p1.x == p2.x) && (p1.y == p2.y) && (p1.z == p2.z) {true} else {false}
}
fn line_is_superposition(ln1: &Line, ln2: &Line) -> bool {
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
    if (k111 * b21 == k211 * b11) && (k112 * b22 == k212 * b12) && 
    (k121 * b22 == k221 * b12) && (k122 * b22 == k222 * b12) {true} else {false}
}
fn plane_is_superposition(pn1: &Plane, pn2: &Plane) -> bool {
    let k11 = pn1.get_func_args()[0];
    let k12 = pn1.get_func_args()[1];
    let k13 = pn1.get_func_args()[2];
    let b1 = pn1.get_func_args()[3];
    let k21 = pn2.get_func_args()[0];
    let k22 = pn2.get_func_args()[1];
    let k23 = pn2.get_func_args()[2];
    let b2 = pn2.get_func_args()[3];
    if (k11 * b2 == k21 * b1) && (k12 * b2 == k22 * b1) && (k13 * b2 == k23 * b1) {true} 
    else {false}
}

