// 3D part
use super::component::*;

/// |k11*y + k12*x = b1
/// |k21*z + k22*x = b2
pub fn calc_line_func(p1: &Point, p2: &Point) -> Option<[[f64; 3]; 2]> {
    if (p2.x != p1.x) || (p2.y != p1.y) {
        if (p2.x != p1.x) || (p2.z != p1.z) {
            return Some([[p2.x - p1.x, p1.y - p2.y, p2.x * p1.y - p1.x * p2.y], 
                        [p2.x - p1.x, p1.z - p2.z, p2.x * p1.z - p1.x * p2.z]]);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

/// k1*x + k2*y + k3*z = b
pub fn calc_plane_func(p1: &Point, p2: &Point, p3: &Point) -> Option<[f64; 4]> {
    let k1 = (p1.x - p2.x) * (p2.y - p3.y) - (p1.y - p2.y) * (p2.x - p3.x);
    let k2 = (p1.y - p2.y) * (p2.z - p3.z) - (p1.z - p2.z) * (p2.y - p3.y);
    let k3 = (p1.z - p2.z) * (p2.x - p3.x) - (p1.x - p2.x) * (p2.z - p3.z);
    let b = p1.x * k2 + p1.y * k3 + p1.z * k1;
    if k1 != 0.0 && k2 != 0.0 && k3 != 0.0 {
        return Some([k1, k2, k3, b]);
    } else {
        return None;
    }
}