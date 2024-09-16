use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul, Div};

use super::tree::*;

pub struct Monomial<'a> {
    var: BTreeMap<&'static str, f64>,
    coef: BTreeMap<&'static str, f64>,
    namespace: &'a NameSpace
}

impl<'a> Monomial<'a> {
    pub fn new(namespace: &'a NameSpace) -> Self {
        let mut coef = BTreeMap::new();
        coef.insert("1", 1.0);
        Self {
            coef,
            var: BTreeMap::new(),
            namespace
        }
    }
    pub fn mul_var(&mut self, v: &'static str) -> &mut Self {
        if self.coef.contains_key(v) {
            self
        } else {
            self.var.entry(v).or_insert(1.0);
            self
        }
    }
    pub fn mul_const(&mut self, c: &'static str) -> &mut Self {
        if self.var.contains_key(c) {
            self
        } else {
            self.coef.entry(c).or_insert(1.0);
            self
        }
    }
    /// set value to some of the constants of the expression
    /// 
    /// notice that the value table may contants other constants that is not contained in current expression
    pub fn set_const_v(&self, v_table: BTreeMap<&'static str, f64>) -> Self {
        let mut new_coef = BTreeMap::new();
        let mut num = *self.coef.get(&"1").unwrap();
        for c in self.coef.keys() {
            match v_table.get(c) {
                Some(const_val) => {
                    num *= *const_val * *self.coef.get(c).unwrap();
                }
                None => {
                    new_coef.insert(*c, *self.coef.get(c).unwrap());
                }
            }
        }
        new_coef.insert("1", num);
        Self {
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
    pub fn new(namespace: &'a NameSpace) -> Self {
        Self {
            expr_tree: ExprTree::new(),
            count_var: BTreeMap::new(),
            count_const: BTreeMap::new(),
            namespace
        }
    }
}
