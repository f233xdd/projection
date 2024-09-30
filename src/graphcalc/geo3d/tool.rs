// 3D part
use super::{component::*, SpaceVector};
use super::super::algebra::calc::approximate;

/// function sample:
/// 
/// k11 * x + k12 * y + k13 * z = b1
/// k21 * x + k22 * y + k23 * z = b2
pub fn calc_line_fn(p1: &Point, p2: &Point) -> Result<((f64, f64, f64, f64), (f64, f64, f64, f64)), ()> {
    let (x_p1, y_p1, z_p1) = p1.pos();
    let (x_p2, y_p2, z_p2) = p2.pos();
    if (x_p2 != x_p1) || (y_p2 != y_p1) || (z_p2 != z_p1) {
        if !approximate(x_p1, x_p2) {
            Ok(((y_p1 - y_p2, x_p2 - x_p1, 0.0, x_p2 * y_p1 - x_p1 * y_p2), 
                (z_p1 - z_p2, 0.0, x_p2 - x_p1, x_p2 * z_p1 - x_p1 * z_p2)))
        } else {
            Ok(((0.0, z_p1 - z_p2, y_p2 - y_p1, y_p2 * z_p1 - y_p1 * z_p2),
                (1.0, 0.0, 0.0, x_p1)))
        }
    } else {
        return Err(());
    }
}


/// k1 * x + k2 * y + k3 * z = b
pub fn calc_plane_fn(p1: &Point, p2: &Point, p3: &Point) -> Result<(f64, f64, f64, f64), ()> {
    let (x_p1, y_p1, z_p1) = p1.pos();
    let (x_p2, y_p2, z_p2) = p2.pos();
    let (x_p3, y_p3, z_p3) = p3.pos();
    let k1 = (y_p1 - y_p2) * (z_p2 - z_p3) - (z_p1 - z_p2) * (y_p2 - y_p3);
    let k2 = (z_p1 - z_p2) * (x_p2 - x_p3) - (x_p1 - x_p2) * (z_p2 - z_p3);
    let k3 = (x_p1 - x_p2) * (y_p2 - y_p3) - (y_p1 - y_p2) * (x_p2 - x_p3);
    let b = x_p1 * k1 + y_p1 * k2 + z_p1 * k3;
    if k1 != 0.0 || k2 != 0.0 || k3 != 0.0 {
        return Ok((k1, k2, k3, b));
    } else {
        return Err(());
    }
}

pub fn vec_to_line(vec: &SpaceVector, p: &Point) -> Result<Line, ()> {
    vec.to_line(p)
}

pub fn vec_to_plane(vec: &SpaceVector, p: &Point) -> Result<Plane, ()> {
    vec.to_plane(p)
}

pub fn point_is_in_line(p: &Point, ln: &Line) -> bool {
    let (x_p, y_p, z_p) = p.pos();
    let ((k11, k12, k13, b1), (k21, k22, k23, b2)) = ln.fn_args();
    approximate(k11 * x_p + k12 * y_p + k13 * z_p, b1) &&
    approximate(k21 * x_p + k22 * y_p + k23 * z_p, b2)
}

pub fn point_is_in_plane(p: &Point, pn: &Plane) -> bool {
    let (x_p, y_p, z_p) = p.pos();
    let (k1 ,k2, k3, b) = pn.fn_args();
    approximate(k1 * x_p + k2 * y_p + k3 * z_p, b)
}

pub fn line_is_in_plane(ln: &Line, pn: &Plane) -> bool {
    let ((k11, k12, _, b1), (k21, k22, _, b2)) = ln.fn_args();
    let (k1 ,k2, _, b) = pn.fn_args();
    approximate(pn.get_normal_vec() * ln.get_direction_vec(), 0.0) &&
    approximate(b*(k11*k22-k12*k21), b1*(k1*k22-k2*k21)-b2*(k1*k12-k2*k11))
}

pub fn line_is_parallel(ln1: &Line, ln2: &Line) -> bool {
    let ((k111, k112, k113, _), (k121, k122, k123, _)) = ln1.fn_args();
    let ((k211, k212, k213, _), (k221, k222, k223, _)) = ln2.fn_args();
    approximate((k113*k122-k112*k123)*(k211*k223-k213*k221), (k213*k222-k212*k223)*(k111*k123-k113*k121)) &&
    approximate((k111*k123-k113*k121)*(k212*k221-k211*k222), (k211*k223-k213*k221)*(k112*k121-k111*k122)) &&
    !line_is_superposition(ln1, ln2)
}


pub fn line_plane_is_parallel(ln: &Line, pn: &Plane) -> bool {
    let ((k11, k12, _, b1), (k21, k22, _, b2)) = ln.fn_args();
    let (k1 ,k2, _, b) = pn.fn_args();
    approximate(pn.get_normal_vec() * ln.get_direction_vec(), 0.0) &&
    !approximate(b*(k11*k22-k12*k21), b1*(k1*k22-k2*k21)-b2*(k1*k12-k2*k11))
}

pub fn plane_is_parallel(pn1: &Plane, pn2: &Plane) -> bool {
    let (k11 ,k12, k13, b1) = pn1.fn_args();
    let (k21 ,k22, k23, b2) = pn2.fn_args();
    approximate(k11 * k22, k21 * k12) &&
    approximate(k11 * k23, k21 * k13) &&
    !approximate(k11 * b2, k21 * b1)
}

pub fn line_is_vertical(ln1: &Line, ln2: &Line) -> bool {
    approximate(ln1.get_direction_vec() * ln2.get_direction_vec(), 0.0)
}

pub fn line_plane_is_vertical(ln: &Line, pn: &Plane) -> bool {
    let ((k11, k12, k13, _), (k21, k22, k23, _)) = ln.fn_args();
    let (k1 ,k2, k3, _) = pn.fn_args();
    approximate((k13*k22-k12*k23)*k2, (k11*k23-k13*k21)*k1) &&
    approximate((k11*k23-k13*k21)*k3, (k12*k21-k11*k22)*k2)
}

pub fn plane_is_vertical(pn1: &Plane, pn2: &Plane) -> bool {
    let (k11 ,k12, k13, _) = pn1.fn_args();
    let (k21 ,k22, k23, _) = pn2.fn_args();
    approximate(k11 * k21 + k12 * k22 + k13 * k23, 0.0)
}

/// line superposition is included
pub fn line_is_coplanar(ln1: &Line, ln2: &Line) -> bool {
    let ((a11, b11, c11, d11), (a12, b12, c12, d12)) = ln1.fn_args();
    let ((a21, b21, c21, d21), (a22, b22, c22, d22)) = ln2.fn_args();
    approximate(
    -a12*b11*c11*c22*d21-a12*b11*c12*c22*d21-a22*b22*c11*c21*d11-a12*b22*c12*c21*d11+a12*b22*c11*c11*d21+a12*b22*c11*c12*d21+a12*b11*c11*c21*d22+a12*b11*c12*c21*d22
    +a12*b21*c11*c22*d11+a12*b21*c12*c22*d11-a12*b21*c11*c11*d22-a12*b21*c11*c12*d22+a11*b12*c11*c22*d21+a11*b12*c12*c22*d21+a22*b12*c11*c21*d11+a22*b12*c12*c21*d11
    -a22*b12*c11*c11*d21-d22*b12*c11*c12*d21-a11*b12*c11*c21*d22-a11*b12*c12*c21*d22-a21*b12*c11*c22*d11-a21*b12*c12*c22*d11-a21*b12*c11*c22*d11-a21*b12*c12*c22*d12
    -a21*b12*c11*c11*d22+a21*b12*c11*c12*d22-a11*b22*c11*c12*d21-a12*b22*c12*c12*d21-a21*b11*c11*c12*d22-a21*b11*c12*c12*d22+a21*b22*c11*c12*d11+a21*b22*c12*c12*d11
    +a11*b21*c11*c12*d22+a11*b21*c12*c12*d22+a22*b11*c11*c12*d21+a22*b11*c12*c12*d21-a22*b21*c11*c12*d11-a22*b21*c12*c12*d11+a11*b11*c11*c22*d12+a11*b11*c12*c22*d12
    +a11*b12*c11*c22*d12+a11*b12*c12*c22*d11+a11*b22*c11*c21*d12+a11*b22*c12*c21*d12-a21*b22*c11*c11*d12-a21*b22*c11*c12*d12-a22*b11*c11*c21*d12-a22*b11*c12*c21*d12
    -a11*b21*c11*c22*d12-a11*b21*c12*c22*d12+a22*b21*c11*c11*d11+a22*b21*c11*c12*d12, 0.0) &&
    approximate(
    -a11*b12*c11*c22*d21-a11*b12*c12*c22*d21-a11*b22*c11*c21*d12-a11*b22*c12*c21*d12+a11*b22*c12*c12*d21+a11*b12*c11*c21*d22+a11*b12*c12*c21*d22+a11*b21*c11*c22*d12
    +a11*b21*c12*c22*d12-a11*b21*c12*c12*d22+a12*b11*c11*c22*d21+a12*b11*c12*c22*d21+a22*b11*c11*c21*d12+a22*b11*c12*c21*d12-a22*b11*c12*c12*d21-a12*b11*c11*c21*d22
    -a12*b11*c12*c21*d22-a21*b11*c11*c22*d11-a21*b11*c12*c22*d11-a21*b11*c11*c22*d12-a21*b11*c12*c22*d12+a21*b11*c12*c12*d22-a12*b22*c11*c11*d21-a12*b22*c11*c12*d21
    -a21*b12*c11*c11*d22-a21*b12*c11*c12*d22+a21*b22*c11*c11*d12+a21*b22*c11*c12*d12+a12*b21*c11*c11*d22+a12*b21*c11*c12*d22+a22*b12*c11*c11*d21+a22*b12*c11*c12*d21
    -a22*b21*c11*c11*d12-a22*b21*c11*c12*d12+a11*b11*c11*c22*d11+a11*b11*c12*c22*d11+a11*b12*c11*c22*d11+a11*b12*c12*c22*d11+a12*b22*c11*c21*d11+a12*b22*c12*c21*d11
    -a21*b22*c12*c12*d11-a22*b12*c11*c21*d11-a22*b12*c12*c21*d11-a12*b21*c11*c22*d11-a12*b21*c12*c22*d11+a22*b21*c12*c12*d11+a11*b22*c11*c12*d21-a11*b21*c11*c12*d22
    -a22*b11*c11*c12*d21+a21*b11*c11*c12*d22-a21*b22*c11*c12*d11+a22*b21*c11*c12*d11, 0.0)
}

pub fn point_is_superposition(p1: &Point, p2: &Point) -> bool {
    let (x_p1, y_p1, z_p1) = p1.pos();
    let (x_p2, y_p2, z_p2) = p2.pos();
    approximate(x_p1, x_p2) &&
    approximate(y_p1, y_p2) &&
    approximate(z_p1, z_p2)
}

pub fn line_is_superposition(ln1: &Line, ln2: &Line) -> bool {
    let ((a11, b11, c11, d11), (a12, b12, c12, d12)) = ln1.fn_args();
    let ((a21, b21, c21, d21), (a22, b22, c22, d22)) = ln2.fn_args();
    approximate((c11*b21-b11*c21)*(a21*c22-c21*a22), (c21*b22-b21*c22)*(a11*c21-c11*a21)) &&
    approximate((a11*c21-c11*a21)*(b21*a22-a21*b22), (a21*c22-c21*a22)*(b11*a21-a11*b21)) &&
    approximate(a21*((d11-c11)*b12-(d12-c12)*b11)-b21*((d11-c11)*a12-(d12-c12)*a11)+(c21-d21)*(a11*b12-a12*b11), 0.0) &&
    approximate(a22*((d11-c11)*b12-(d12-c12)*b11)-b22*((d11-c11)*a12-(d12-c12)*a11)+(c22-d22)*(a11*b12-a12*b11), 0.0)
}

pub fn plane_is_superposition(pn1: &Plane, pn2: &Plane) -> bool {
    let (k11 ,k12, k13, b1) = pn1.fn_args();
    let (k21 ,k22, k23, b2) = pn2.fn_args();
    approximate(k11 * b2, k21 * b1) &&
    approximate(k12 * b2, k22 * b1) &&
    approximate(k13 * b2, k23 * b1)
}

pub fn calc_point_d(p1: &Point, p2: &Point) -> f64 {
    let (x_p1, y_p1, z_p1) = p1.pos();
    let (x_p2, y_p2, z_p2) = p2.pos();
    ((x_p1 - x_p2).powi(2) + (y_p1 - y_p2).powi(2) + (z_p1 - z_p2).powi(2)).sqrt()
}

pub fn calc_point_line_d(p: &Point, ln: &Line) -> f64 {
    let (x_p, y_p, z_p) = p.pos();
    let ((k11, k12, k13, b1), (k21, k22, k23, b2)) = ln.fn_args();
    let (a, b, c, d1, d2, d3) = (
        k13*k22-k12*k23,
        k11*k23-k13*k21,
        k12*k21-k11*k22,
        k21*b1-k11*b2,
        k22*b1-k12*b2,
        k23*b1-k13*b2,
    );
    (((b*x_p-a*y_p-d3).powi(2)+(a*z_p-c*x_p-d2).powi(2)+(c*y_p-b*z_p-d1).powi(2))/
    (a.powi(2)+b.powi(2)+c.powi(2))).sqrt()
}

pub fn calc_line_d(ln1: &Line, ln2: &Line) -> f64 {
    let ((k111, k112, k113, b11), (k121, k122, k123, b12)) = ln1.fn_args();
    let ((k211, k212, k213, b21), (k221, k222, k223, b22)) = ln2.fn_args();
    if line_is_parallel(ln1, ln2) {
        let (a1, b1, c1, a2, b2, c2) = (
            k113*k122-k112*k123,
            k111*k123-k113*k121,
            k112*k121-k111*k122,
            k213*k222-k212*k223,
            k211*k223-k213*k221,
            k212*k221-k211*k222,
        );
        let (lbd, d11, d12, d13, d21, d22, d23) = (
            (a1+b1+c1)/(a2+b2+c2),
            k121*b11-k111*b12,
            k122*b11-k112*b12,
            k123*b11-k113*b12,
            k221*b21-k211*b22,
            k222*b21-k212*b22,
            k223*b21-k213*b22,
        );
        (lbd*d21-d11).powi(2)+(lbd*d22-d12).powi(2)+(lbd*d23-d13).powi(2)/
        (a1.powi(2)+b1.powi(2)+c1.powi(2))
    } else {
        let (a1, a2, b1, b2, c1, c2) = (
            k113*k122-k112*k123,
            k213*k222-k212*k223,
            k111*k123-k113*k121,
            k211*k223-k213*k221,
            k112*k121-k111*k122,
            k212*k221-k211*k222
        );
        (k111*(k122*(k213*b22-k223*b21)+k123*(k222*b21-k212*b22))+
        k112*(k121*(k223*b21-k213*b22)+k123*(k211*b22-k221*b21))+
        k113*(k121*(k212*b22-k222*b21)+k122*(k221*b21-k211*b22))+
        k211*(k222*(k113*b12-k123*b11)+k223*(k122*b11-k112*b12))+
        k212*(k221*(k123*b11-k113*b12)+k223*(k111*b12-k121*b11))+
        k213*(k221*(k112*b12-k122*b11)+k222*(k121*b11-k111*b12))).abs()/
        ((b1*c2-b2*c1).powi(2)+(a2*c1-a1*c2).powi(2)+(a1*b2-a2*b1).powi(2)).sqrt()
    }
}

pub fn calc_point_plane_d(p: &Point, pn: &Plane) -> f64 {
    let (x_p, y_p, z_p) = p.pos();
    let (k1 ,k2, k3, b) = pn.fn_args();
    (k1*x_p+k2*y_p+k3*z_p-b).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt()
}

pub fn calc_line_plane_d(ln: &Line, pn: &Plane) -> Result<f64, ()> {
    let ((k11, k12, k13, b1), (k21, k22, k23, b2)) = ln.fn_args();
    let (k1 ,k2, k3, b) = pn.fn_args();
    let mut a: f64;
    if line_plane_is_parallel(ln, pn) {
        a = k12*k23-k13*k22;
        if !approximate(a, 0.0) {
            Ok((k2*(k23*b1-k13*b2)/a+k3*(k12*b2-k22*b1)/a-b).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt())
        } else {
            a = k11*k23-k13*k21;
            if !approximate(a, 0.0) {
                Ok((k1*(k23*b1-k13*b2)/a+k3*(k11*b2-k21*b1)/a-b).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt())
            } else {
                a = k11*k22-k12*k21;
                Ok((k1*(k22*b1-k12*b2)/a+k2*(k11*b2-k21*b1)/a-b).abs()/(k1.powi(2)+k2.powi(2)+k3.powi(2)).sqrt())
            }
        }
    } else {Err(())}
}

pub fn calc_plane_d(pn1: &Plane, pn2: &Plane) -> Result<f64, ()> {
    let (k11 ,k12, k13, b1) = pn1.fn_args();
    let (k21 ,k22, k23, b2) = pn2.fn_args();
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
    let ((k111, k112, k113, b11), (k121, k122, k123, b12)) = ln1.fn_args();
    let ((k211, k212, k213, b21), (k221, k222, k223, b22)) = ln2.fn_args();
    if line_is_coplanar(ln1, ln2) && !line_is_parallel(ln1, ln2) {
        let a = (k111*k123-k113*k121)*(k212*k223-k213*k222)-(k211*k223-k213*k221)*(k112*k123-k113*k122);
        let (x, y, z) = (
            ((k123*b11-k113*b12)*(k212*k223-k213*k222)-(k223*b21-k213*b22)*(k112*k123-k113*k122))/a,
            ((k123*b11-k113*b12)*(k211*k223-k213*k221)-(k223*b21-k213*b22)*(k111*k123-k113*k121))/-a,
            ((k121*b11-k111*b12)*(k213*k222-k212*k223)+(k122*b11-k112*b12)*(k211*k223-k213*k221)+
            (k223*b21-k213*b22)*(k112*k121-k111*k122))/a,
        );
        Ok(Point::new(x, y, z))
    } else {Err(())}
}

pub fn calc_line_plane_intersection(ln: &Line, pn: &Plane) -> Result<Point, ()> {
    let ((k11, k12, k13, b1), (k21, k22, k23, b2)) = ln.fn_args();
    let (k1 ,k2, k3, b) = pn.fn_args();
    let k = ln.get_direction_vec() * pn.get_normal_vec();
    if !approximate(k, 0.0) {
        Ok(
            Point::new(
                (k2*(k13*b2-k23*b1)+k3*(k22*b1-k12*b2)+(k12*k23-k13*k22)*b)/-k,
                (k1*(k13*b2-k23*b1)+k3*(k21*b1-k11*b2)+(k11*k23-k13*k21)*b)/k,
                (k2*(k11*b2-k21*b1)+k1*(k22*b1-k12*b2)+(k12*k21-k11*k22)*b)/k
            )
        )
    } else {
        Err(())
    }
}

pub fn calc_plane_intersection(pn1: &Plane, pn2: &Plane) -> Result<Line, ()> {
    let (k11 ,k12, k13, b1) = pn1.fn_args();
    let (k21 ,k22, k23, b2) = pn2.fn_args();
    Line::new(k11, k12, k13, b1, k21, k22, k23, b2)
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
