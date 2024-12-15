// this is not done yet
mod tree;
mod expr;
pub mod lexer;
pub mod calc;

pub use tree::{NameSpace, MathFn, Node};

pub enum Num<A, R> {
    Algebra(A),
    RealNum(R),
}
