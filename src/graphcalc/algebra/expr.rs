use std::collections::HashMap;

use super::tree::*;

pub struct Monomial<'a> {
    var: HashMap<&'a str, f64>,
    coef: HashMap<&'a str, f64>,
}

pub struct IntExpr<'a> {
    expr_tree: ExprTree<'a>,
    count_var: HashMap<&'a str, f64>,
    count_const: HashMap<&'a str, f64>,
}