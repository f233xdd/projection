mod tree;
mod expr;
pub mod calc;

pub enum Num<A, R> {
    Algebra(A),
    RealNum(R),
}
