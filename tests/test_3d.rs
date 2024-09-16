// Author: Max
use std::f64::consts::PI;

use projection::graphcalc::geo3d::{tool::*, feature::Superposition};
use projection::{p, ln, pn, vector};

const ACCURACY: f64 = 1e-8;

pub fn test_main() {
    test_vec_calc();
    test_func_calc();
    test_is_in();
    test_is_parallel();
    test_is_vertical();
    test_line_pos();
    test_calc_d();
    test_calc_angle();
    test_calc_intersection();
}

#[test]
fn test_vec_calc() {
    let a = vector!(3.0, 4.0, 3.0);
    let b = vector!(-6.0, 3.0, 2.0);
    assert!(&a+&b == vector!(-3.0, 7.0, 5.0));
    assert!(&a-&b == vector!(9.0, 1.0, 1.0));
    assert!(&a*&b == 0.0);
    assert!(&a%&b == vector!(-1.0, -24.0, 33.0));
}

#[test]
fn test_func_calc() {
    let ln = ln!((1.0, 0.0, 1.0), (0.0, 1.0, 0.0)).unwrap();
    let ln1 = ln!((k11: -1.0, k12: -1.0, k13: 0.0, b1: -1.0), (k21: -1.0, k22: 0.0, k23: 1.0, b2: 0.0)).unwrap();
    println!("{}\n{}", ln, ln1);
    assert!(ln.is_superposition(&ln1));
    assert!(pn!((0.0, 0.0, 0.0), (1.0, 0.0, 1.0), (0.0, 1.0, 1.0)).unwrap().is_superposition(&pn!(k1:1.0, k2:1.0, k3:-1.0, b:0.0).unwrap()));
}

#[test]
fn test_is_in() {
    let p = p!(0.5, 0.5, 0.5);
    let ln = ln!((0.0, 0.0, 1.0/3.0), (1.0, 1.0, 2.0/3.0)).unwrap();
    let pn = pn!((0.0, 0.0, 1.0/3.0), (1.0, 1.0, 2.0/3.0), (1.0, 0.0, 1.0/2.0)).unwrap();
    println!("{}",ln);
    assert!(point_is_in_line(&p!(1.0, 1.0, 2.0/3.0), &ln));
    assert!(point_is_in_plane(&p, &pn));
    assert!(line_is_in_plane(&ln, &pn));
}

#[test]
fn test_is_parallel() {
    let ln1 = ln!((1.0, 0.0, 0.0), (0.0, 1.0, 1.0)).unwrap();
    let ln2 = ln!((1.0, 1.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(line_is_parallel(&ln1, &ln2));

    let ln = ln!((0.5, 0.5, 1.0), (1.0, 1.0, 0.0)).unwrap();
    let pn = pn!((1.0, 1.0, 1.0), (1.0, 2.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(line_plane_is_parallel(&ln, &pn));

    let pn1 = pn!((1.0, 0.0, 1.0), (1.0, 1.0, 0.0), (0.0, 1.0, 1.0)).unwrap();
    let pn2 = pn!((1.0, 1.0, 1.0), (1.0, 2.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(plane_is_parallel(&pn1, &pn2));
}

#[test]
fn test_is_vertical() {
    let ln1 = ln!((0.0, 0.0, 1.0), (1.0, 1.0, 0.0)).unwrap();
    let ln2 = ln!((1.0, 2.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(!line_is_vertical(&ln1, &ln2));
    let ln3 = ln!((1.0, 1.0, 0.0), (1.0, 2.0, 1.0)).unwrap();
    assert!(line_is_vertical(&ln1, &ln3));

    let ln = ln!((0.0, 1.0, 0.0), (1.0, 2.0, 1.0)).unwrap();
    let pn = pn!((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)).unwrap();
    assert!(line_plane_is_vertical(&ln, &pn));

    let pn1 = pn!((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)).unwrap();
    let pn2 = pn!((1.0, 2.0, 1.0), (1.0, 2.0, 0.0), (0.0, 1.0, 0.0)).unwrap();
    assert!(plane_is_vertical(&pn1, &pn2));
}

#[test]
fn test_line_pos() {
    let ln1 = ln!((0.0, 0.0, 1.0), (1.0, 2.0, 0.0)).unwrap();
    let ln2 = ln!((1.0, 0.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(line_is_coplanar(&ln1, &ln2));

    let ln3 = ln!((0.0, 0.0, 1.0), (1.0, 1.0, 0.0)).unwrap();
    let ln4 = ln!((-1.0, -1.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(!line_is_coplanar(&ln3, &ln4));
}

#[test]
fn test_calc_d() {
    let p1 = p!(0.0, 1.0, 1.0);
    let ln1 = ln!((0.0, 0.0, 1.0), (1.0, 2.0, 0.0)).unwrap();
    println!("{} {}", calc_point_line_d(&p1, &ln1), 3.0_f64.sqrt()/3.0);
    assert!((calc_point_line_d(&p1, &ln1) - 3.0_f64.sqrt()/3.0).abs() < ACCURACY);

    let ln2 = ln!((1.0 ,0.0 ,1.0), (0.0, 2.0, 1.0)).unwrap();
    let ln3 = ln!((0.0, 0.0, 0.0), (1.0, 2.0, 0.0)).unwrap();
    assert!((calc_line_d(&ln2, &ln3) - 1.0).abs() < ACCURACY);

    let p2 = p!(1.0, 1.0, 1.0);
    let pn1 = pn!((1.0 ,0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)).unwrap();
    assert!((calc_point_plane_d(&p2, &pn1) - 2.0*3.0_f64.sqrt()/3.0).abs() < ACCURACY);

    let ln4 = ln!((1.0 ,1.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    let pn2 = pn!((1.0, 0.0, 0.0), (0.0, 1.0, 1.0), (0.0, 2.0, 0.0)).unwrap();
    assert!((calc_line_plane_d(&ln4, &pn2).unwrap() - 0.4082482904639).abs() < ACCURACY);

    let pn3 = pn!((1.0, 1.0, 0.0), (1.0, 0.0, 1.0), (0.0, 1.0, 1.0)).unwrap();
    let pn4 = pn!((1.0, 1.0, 1.0), (1.0, 2.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    assert!((calc_plane_d(&pn3, &pn4).unwrap() - 3.0_f64.sqrt()/3.0).abs() < ACCURACY);
}

#[test]
fn test_calc_angle() {
    let ln1 = ln!((1.0, 0.0, 1.0), (1.0, 1.0, 0.0)).unwrap();
    let ln2 = ln!((1.0, 0.0, 1.0), (0.0, 2.0, 0.0)).unwrap();
    println!("{} {}", calc_line_angle(&ln1, &ln2), 0.5235987756);
    assert!((calc_line_angle(&ln1, &ln2) - 0.5235987756).abs() < ACCURACY);

    let ln = ln!((1.0, 1.0, 0.0), (1.0, 2.0, 1.0)).unwrap();
    let pn = pn!((1.0, 0.0, 1.0), (1.0, 1.0, 0.0), (0.0, 1.0, 1.0)).unwrap();
    assert!((calc_line_plane_angle(&ln, &pn) - 0.9553166181).abs() < ACCURACY);

    let pn1 = pn!((1.0, 0.0, 0.0), (0.0, 2.0, 0.0), (0.0, 1.0, 1.0)).unwrap();
    let pn2 = pn!((1.0, 0.0, 0.0), (0.0, 2.0, 0.0), (0.0, 0.0, 0.0)).unwrap();
    assert!((calc_plane_angle(&pn1, &pn2) - (PI-1.9913306621)).abs() < ACCURACY);
}

#[test]
fn test_calc_intersection() {
    let ln1 = ln!((1.0, 0.0, 0.0), (0.0, 2.0, 1.0)).unwrap();
    let ln2 = ln!((0.0, 0.0, 1.0), (1.0, 2.0, 0.0)).unwrap();
    assert!(calc_line_intersection(&ln1, &ln2).unwrap().is_superposition(&p!(1.0/2.0, 1.0, 1.0/2.0)));

    let ln = ln!((0.0, 1.0, 1.0), (1.0, 2.0, 0.0)).unwrap();
    let pn = pn!((0.0, 1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 2.0, 1.0)).unwrap();
    assert!(calc_line_plane_intersection(&ln, &pn).unwrap().is_superposition(&p!(1.0/3.0, 4.0/3.0, 2.0/3.0)));

    let pn1 = pn!((0.0, 1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 2.0, 1.0)).unwrap();
    let pn2 = pn!((0.0, 1.0, 1.0), (1.0, 1.0, 0.0), (0.0, 2.0, 0.0)).unwrap();
    let (i, j) = (
        ln!((k11:1.0, k12:1.0, k13:0.0, b1:3.0/2.0), (k21:0.0, k22:0.0, k23:1.0, b2:1.0/2.0)).unwrap(),
        calc_plane_intersection(&pn1, &pn2).unwrap()
    );
    println!("{i}\n{j}");
    assert!(calc_plane_intersection(&pn1, &pn2).unwrap().is_superposition(
        &ln!((k11:1.0, k12:1.0, k13:0.0, b1:3.0/2.0), (k21:0.0, k22:0.0, k23:1.0, b2:1.0/2.0)).unwrap()
    ))
}