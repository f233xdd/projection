use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul, Div};

use super::tree::*;

pub struct Monomial<'a> {
    var: BTreeMap<&'static str, f64>,
    coef: BTreeMap<&'static str, f64>,
    namespace: &'a NameSpace
}

impl<'a> Monomial<'a> {
    pub fn new(namespace: &'a NameSpace) -> Monomial<'a> {  // TODO: init var and const
        Monomial {
            var: BTreeMap::new(),
            coef: BTreeMap::new(),
            namespace
        }
    }
    pub fn set_const_v(&self, v_table: BTreeMap<&'static str, f64>) -> Monomial<'a> {
        let mut new_coef = BTreeMap::new();
        let mut num = *self.coef.get(&"1").unwrap();
        for c in self.coef.keys() {
            match v_table.get(c) {
                Some(const_val) => {
                    num *= *const_val * *self.coef.get(c).unwrap();
                }
                None => {
                    new_coef.entry(*c).or_insert(*self.coef.get(c).unwrap());
                }
            }
        }
        new_coef.entry("1").or_insert(num);
        Monomial {
            var: self.var.clone(),
            coef: new_coef,
            namespace: self.namespace
        }
    }
    // pub fn into_expr() -> Expr {}
}

pub struct Expr<'a> {
    expr_tree: ExprTree<'a>,
    count_var: BTreeMap<&'a str, f64>,
    count_const: BTreeMap<&'a str, f64>,
    namespace: &'a NameSpace
}

impl<'a> Expr<'a> {
    pub fn new(namespace: &'a NameSpace) -> Expr {
        Expr {
            expr_tree: ExprTree::new(),
            count_var: BTreeMap::new(),
            count_const: BTreeMap::new(),
            namespace
        }
    }
}
