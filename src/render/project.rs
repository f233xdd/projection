use std::f64::consts::PI;

use super::{graphcalc, p, pn, approximate};
use graphcalc::{
    geo2d, geo3d, interface::{CalcAngle, CalcDistance},
};

/// theta in radians
pub fn single_project(
    sight: &geo3d::Point,
    target: &geo3d::Point,
    r0: f64,
    theta_xoy: f64,
    theta_z: f64
) -> Option<geo2d::Point>
{
    let (x_o, y_o, z_o) = sight.pos();
    let (x_p, y_p, z_p) = target.pos();
    let (x_s, y_s, z_s) = (
        r0 * theta_z.cos() * theta_xoy.cos() + x_o,
        r0 * theta_z.cos() * theta_xoy.sin() + y_o,
        r0 * theta_z.sin() + z_o,
    );
    if pn!(k1: x_s, k2: y_s, k3: z_s, b: r0*(x_s.powi(2)+y_s.powi(2)+z_s.powi(2)).sqrt())
        .unwrap()
        .is_on_same_side(sight, target) { None }
    else {
        let k_s = x_s * x_p + y_s * y_p + z_s * z_p;
        if k_s == 0.0 {
            None
        } else {
            let (x_s1, y_s1, z_s1) = (
                r0.powi(2) * x_p / k_s,
                r0.powi(2) * y_p / k_s,
                r0.powi(2) * z_p / k_s,
            );
            let d = p!(x_s1, y_s1, z_s1).calc_d(&p!(x_s, y_s, z_s));
            let x: f64;
            let mut y = 0.0;
            let mut spec = false;
            if !approximate(x_s, 0.0) && !approximate(y_s, 0.0) {
                if (x_s > 0.0 && y_s > 0.0) || (x_s < 0.0 && y_s < 0.0) {
                    x = d * cal_cos_by_point(
                        &p!(x_s/2.0, x_s.powi(2)/(2.0*y_s)+y_s, z_s),
                        &p!(x_s1, y_s1, z_s1),
                        &p!(x_s, y_s, z_s)
                    );
                } else {
                    x = d * cal_cos_by_point(
                        &p!(y_s.powi(2)/(2.0*x_s)+x_s, y_s/2.0, z_s),
                        &p!(x_s1, y_s1, z_s1),
                        &p!(x_s, y_s, z_s)
                    );
                }
            } else if approximate(y_s, 0.0) && !approximate(x_s, 0.0) {
                if x_s > 0.0 {
                    x = d * cal_cos_by_point(
                        &p!(x_s, 1.0, z_s), 
                        &p!(x_s1, y_s1, z_s1), 
                        &p!(x_s, y_s, z_s)
                    );
                } else {
                    x = d * cal_cos_by_point(
                        &p!(x_s, -1.0, z_s), 
                        &p!(x_s1, y_s1, z_s1), 
                        &p!(x_s, y_s, z_s)
                    );
                }
            } else if !approximate(y_s, 0.0) && approximate(x_s, 0.0) {
                if y_s > 0.0 {
                    x = d * cal_cos_by_point(
                        &p!(-1.0, y_s, z_s), 
                        &p!(x_s1, y_s1, z_s1), 
                        &p!(x_s, y_s, z_s)
                    );
                } else {
                    x = d * cal_cos_by_point(
                        &p!(1.0, y_s, z_s), 
                        &p!(x_s1, y_s1, z_s1), 
                        &p!(x_s, y_s, z_s)
                    );
                }
            } else {
                println!("{x_s}, {y_s}");
                let mut target = p!(x_s1, y_s1);
                spec = true;
                if z_s > 0.0 {
                    geo2d::tool::rotate(&p!(0.0, 0.0), &mut target, PI-theta_xoy);
                    (y, x) = target.pos();
                } else {
                    geo2d::tool::rotate(&p!(0.0, 0.0), &mut target, PI/2.0-theta_xoy);
                    (x, y) = target.pos();
                }
            }
            if !spec {
                if z_s > 0.0 {
                    y = -d * cal_cos_by_point(
                        &p!(x_s+z_s.powi(2)*x_s/(x_s.powi(2)+y_s.powi(2)), y_s+z_s.powi(2)*y_s/(x_s.powi(2)+y_s.powi(2)), 0.0), 
                        &p!(x_s1, y_s1, z_s1), 
                        &p!(x_s, y_s, z_s));
                } else if z_s < 0.0 {
                    y = d * cal_cos_by_point(
                        &p!(x_s+z_s.powi(2)*x_s/(x_s.powi(2)+y_s.powi(2)), y_s+z_s.powi(2)*y_s/(x_s.powi(2)+y_s.powi(2)), 0.0), 
                        &p!(x_s1, y_s1, z_s1), 
                        &p!(x_s, y_s, z_s));
                } else {
                    y = z_s1;
                }
            } else {}
            Some(p!(x, y))
        }
    }



}

fn cal_cos_by_point(s1: &geo3d::Point, s2: &geo3d::Point, o: &geo3d::Point) -> f64 {
    geo3d::Line::from_p(s1, o)
        .unwrap()
        .calc_angle(&geo3d::Line::from_p(s2, o).unwrap())
        .cos()
}
