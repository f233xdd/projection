// this mod is not done yet
pub mod calc;
pub mod err;
mod expr;
pub mod lexer;
mod tree;
pub mod vector;

pub use tree::{MathFn, NameSpace, Node};

pub enum Num<A, R> {
    Algebra(A),
    RealNum(R),
}
