// Co-author: Equar, Max
// I'd appreciate my friend Equar for his great effor to those test cases.
// All the cases given were calculated by him.
// Again, thank him veru much.
// However, it's a pity that he can't write code by Rust, so I did all these things.
use std::f64::consts::PI;

use crate::geo2d;
use crate::geo2d::tool::*;
use geo2d::Point as p;
use geo2d::Line as ln;


pub fn test_main() {
    test_calc_line_func();
    test_is_in();
    test_is_parallel();
    test_is_vertical();
    test_is_superposition();
    test_calc_point_line_d();
    test_calc_line_d();
    test_calc_angle();
    test_calc_intersection();
}

fn test_calc_line_func() {
    let v = calc_line_func(&p{x: 9.0, y: 52.0}, &p{x: 5.0 ,y: 32.0}).unwrap();
    assert_eq!(v, [-4.0,20.0,-28.0]);
}

fn test_is_in() {
    let p1 = p::new(5.0, 9.0); // out
    let p2 = p::new(5.0, 55.0); // in
    let p3 = p::new(4.0, 46.0); // in
    let p4 = p::new(3.0, 37.0); // in
    let l = ln::new(&p2, &p3).unwrap();
    assert_eq!(is_in(&p1, &l), false);
    assert_eq!(is_in(&p4, &l), true);
}

fn test_is_parallel() {
    let p1 = p::new(0.0, 0.0);
    let p2 = p::new(3.0, 3.0);
    let p3 = p::new(0.0, 1.0);
    let p4 = p::new(3.0, 4.0);
    let p5 = p::new(5.0, 9.0);
    let p6 = p::new(10.0, 41.0);
    let ln1 = ln::new(&p1, &p2).unwrap();
    let ln2 = ln::new(&p3, &p4).unwrap();
    let ln3 = ln::new(&p5, &p6).unwrap();
    assert_eq!(is_parallel(&ln1, &ln2), true);
    assert_eq!(is_parallel(&ln1, &ln3), false);
}

fn test_is_vertical() {
    let p1 = p::new(1.0, 14.0);
    let p2 = p::new(2.0, 19.0);
    let p3 = p::new(5.0, 7.0);
    let p4 = p::new(10.0, 6.0);
    let p5 = p::new(0.0, 0.0);
    let p6 = p::new(1.0, 1.0);
    let ln1 = ln::new(&p1, &p2).unwrap();
    let ln2 = ln::new(&p3, &p4).unwrap();
    let ln3 = ln::new(&p5, &p6).unwrap();
    assert_eq!(is_vertical(&ln1, &ln2), true);
    assert_eq!(is_vertical(&ln1, &ln3), false);
}

fn test_is_superposition() {
    let p1 = p::new(0.0, 0.0);
    let p2 = p::new(1.0, 1.0);
    let p3 = p::new(5.0, 5.0);
    let p4 = p::new(4.0, 4.0);
    let p5 = p::new(4.0, 8.0);
    let p6 = p::new(5.0,9.0);
    let ln1 = ln::new(&p1, &p2).unwrap();
    let ln2 = ln::new(&p3, &p4).unwrap();
    let ln3 = ln::new(&p5, &p6).unwrap();
    assert_eq!(line_is_superposition(&ln1, &ln2), true);
    assert_eq!(line_is_superposition(&ln1, &ln3), false);
}

fn test_calc_point_line_d() {
    let p1 = p::new(5.0, 4.0);
    let p2 = p::new(6.0, 4.0);
    let p3 = p::new(4.0, -1.0);
    let l = ln::new(&p1, &p2).unwrap();
    assert_eq!(calc_point_line_d(&p3, &l), 5.0);
}

fn test_calc_line_d() {
    let p1 = p::new(5.0, 10.0);
    let p2 = p::new(6.0, 11.0);
    let p3 = p::new(8.0, 5.0);
    let p4 = p::new(9.0, 6.0);
    let ln1 = ln::new(&p1, &p2).unwrap();
    let ln2 = ln::new(&p3, &p4).unwrap();
    assert_eq!(calc_line_d(&ln1, &ln2).unwrap(), 5.65685424949238);
}

fn test_calc_angle() {
    let p1 = p::new(8.0, 5.0);
    let p2 = p::new(9.0, 6.0);
    let p3 = p::new(1.0, 1.0);
    let p4 = p::new(5.0, 1.0);
    let ln1 = ln::new(&p1, &p2).unwrap();
    let ln2 = ln::new(&p3, &p4).unwrap();
    assert!(calc_angle(&ln1, &ln2) - PI/4.0 < 0.000000000000001);
}

fn test_calc_intersection() {
    let p1 = p::new(4.0, 8.0);
    let p2 = p::new(5.0, 10.0);
    let p3 = p::new(0.0, 5.0);
    let p4 = p::new(5.0, 0.0);
    let ln1 = ln::new(&p1, &p2).unwrap();
    let ln2 = ln::new(&p3, &p4).unwrap();
    let cross = calc_intersection(&ln1, &ln2).unwrap();
    assert_eq!(point_is_superposition(&cross, &p::new(5.0/3.0, 10.0/3.0)), true);
}
