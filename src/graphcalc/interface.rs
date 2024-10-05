pub trait Superposition<T> {
    fn is_superposition(&self, cpt: &T) -> bool;
}
pub trait Contain<T> {
    fn is_included(&self, cpt: &T) -> bool;
}
pub trait Parallel<T> {
    fn is_parallel(&self, cpt: &T) -> bool;
}
pub trait Vertical<T> {
    fn is_vertical(&self, cpt: &T) -> bool;
}
pub trait Coplanar<T> {
    fn is_coplanar(&self, cpt: &T) -> bool;
}   
pub trait CalcDistance<T, R> {
    fn calc_d(&self, cpt: &T) -> R;
}
pub trait CalcAngle<T> {
    fn calc_angle(&self, cpt: &T) -> f64;
}
pub trait CalcIntersection<T, R> {
    fn calc_intersection(&self, cpt: &T) -> R;
}
