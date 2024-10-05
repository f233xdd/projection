use projection::{p, ln};
use projection::graphcalc::interface::CalcDistance;

fn main() {
    let p = p!(1.0, 2.0);
    let ln = ln!(k1: 1.0, k2: 3.0, b: -5.0).unwrap();
    println!("{}", p.calc_d(&ln))
}
