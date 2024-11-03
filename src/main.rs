use std::f64::consts::PI;

use projection::graphcalc::algebra::NameSpace;
use projection::{define, ln, p};
use projection::graphcalc::interface::CalcDistance;
use projection::render::project::single_project;

fn main() {
    let p = p!(1.0, 2.0);
    let ln = ln!(k1: 1.0, k2: 3.0, b: -5.0).unwrap();
    println!("{}", p.calc_d(&ln));
    println!("{}", single_project(&p!(0.0, 0.0, 0.0), &p!(3.0, 4.0, 3.0), 3.0, PI/4.0, PI/2.0).unwrap());
    let mut np = NameSpace::new();
    define! {
        using namespace np;
        let x, y, z: var;
        let a, b, c: param;
        fn f64::sin => (f64);
        fn f64::cos => (f64);
        fn f64::log => (f64, f64);
        let λ, ζ, ε: param;
        let θ, β, φ: var;
    };
    println!("{:#?}", np);
    use std::cell::{Cell, RefCell};
    let cell = Cell::new(1);
    cell.set(5);
    println!("{:?}", cell);
    let ref_cell = RefCell::new(String::new());
    ref_cell.borrow_mut().push_str("hello");
    println!("{:?}", ref_cell);
}
