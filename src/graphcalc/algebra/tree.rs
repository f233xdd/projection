use super::expr::*;

enum FnType<'a> {
    InitFn(fn(f64, f64) -> f64, IntExpr<'a>),
    Fn(fn(f64, f64) -> f64),
}

enum Node<'a> {
    Add{
        mono: Vec<Monomial<'a>>,
        func: Vec<FnType<'a>>,
        coef: f64,
        next: Box<Node<'a>>,
        supr: &'a Node<'a>,
    },
    Mul{
        mono: Monomial<'a>,
        func: Vec<FnType<'a>>,
        coef: f64,
        next: Box<Node<'a>>,
        supr: &'a Node<'a>,
    },
}

pub struct ExprTree<'a> {
    root: Node<'a>,
    pos: i32,
}