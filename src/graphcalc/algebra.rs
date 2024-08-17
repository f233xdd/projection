mod tree;
mod expr;

pub enum Num<A, R> {
    Algebra(A),
    RealNum(R),
}