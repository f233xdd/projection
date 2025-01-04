// Co-author: Equar, Max
use std::f64::consts::PI;

use projection::graphcalc::geo2d::tool::*;
use projection::{ln, p};

// static TEST_LIST: [fn()->();9] = [
//     test_line_func,
//     test_is_in,
//     test_is_parallel,
//     test_is_vertical,
//     test_is_superposition,
//     test_calc_point_line_d,
//     test_calc_line_d,
//     test_calc_angle,
//     test_calc_intersection,
// ];
#[test]
fn test_line_func() {
    let v = calc_line_fn(&p!(9.0, 52.0), &p!(5.0, 32.0)).unwrap();
    assert_eq!(v, (20.0, -4.0, -28.0));
}

#[test]
fn test_is_in() {
    let p1 = p!(5.0, 9.0); // out
    let p4 = p!(3.0, 37.0); // in
    let l = ln!((5.0, 55.0), (4.0, 46.0)).unwrap();
    assert!(!is_in(&p1, &l));
    assert!(is_in(&p4, &l));
}

#[test]
fn test_is_parallel() {
    let ln1 = ln!((0.0, 0.0), (3.0, 3.0)).unwrap();
    let ln2 = ln!((0.0, 1.0), (3.0, 4.0)).unwrap();
    let ln3 = ln!((5.0, 9.0), (10.0, 41.0)).unwrap();
    assert!(is_parallel(&ln1, &ln2));
    assert!(!is_parallel(&ln1, &ln3));
}

#[test]
fn test_is_vertical() {
    let ln1 = ln!((1.0, 14.0), (2.0, 19.0)).unwrap();
    let ln2 = ln!((5.0, 7.0), (10.0, 6.0)).unwrap();
    let ln3 = ln!((0.0, 0.0), (1.0, 1.0)).unwrap();
    assert_eq!(is_vertical(&ln1, &ln2), true);
    assert_eq!(is_vertical(&ln1, &ln3), false);
}

#[test]
fn test_is_superposition() {
    let ln1 = ln!((0.0, 0.0), (1.0, 1.0)).unwrap();
    let ln2 = ln!((5.0, 5.0), (4.0, 4.0)).unwrap();
    let ln3 = ln!((4.0, 8.0), (5.0, 9.0)).unwrap();
    assert_eq!(line_is_superposition(&ln1, &ln2), true);
    assert_eq!(line_is_superposition(&ln1, &ln3), false);
}

#[test]
fn test_calc_point_line_d() {
    let p3 = p!(4.0, -1.0);
    let l = ln!((5.0, 4.0), (6.0, 4.0)).unwrap();
    assert_eq!(calc_point_line_d(&p3, &l), 5.0);
}

#[test]
fn test_calc_line_d() {
    let ln1 = ln!((5.0, 10.0), (6.0, 11.0)).unwrap();
    let ln2 = ln!((8.0, 5.0), (9.0, 6.0)).unwrap();
    assert_eq!(calc_line_d(&ln1, &ln2).unwrap(), 5.65685424949238);
}

#[test]
fn test_calc_angle() {
    let ln1 = ln!((8.0, 5.0), (9.0, 6.0)).unwrap();
    let ln2 = ln!((1.0, 1.0), (5.0, 1.0)).unwrap();
    assert!((calc_angle(&ln1, &ln2) - PI / 4.0).abs() < 0.0000001);
}

#[test]
fn test_calc_intersection() {
    let ln1 = ln!((4.0, 8.0), (5.0, 10.0)).unwrap();
    let ln2 = ln!((0.0, 5.0), (5.0, 0.0)).unwrap();
    let cross = calc_intersection(&ln1, &ln2).unwrap();
    assert_eq!(
        point_is_superposition(&cross, &p!(5.0 / 3.0, 10.0 / 3.0)),
        true
    );
}
