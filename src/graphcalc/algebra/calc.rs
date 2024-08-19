pub const ACCURENCY: f64 = 1e-8;

pub fn approximate(v1: f64, v2: f64) -> bool {
    (v1-v2).abs() <= ACCURENCY
}