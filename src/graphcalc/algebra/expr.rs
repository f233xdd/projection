use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul, Div};
use std::ptr::eq;

use super::tree::*;

#[derive(Clone)]
pub struct Monomial<'a> {
    vars: BTreeMap<&'a str, f64>,
    params: BTreeMap<&'a str, f64>,
    consts: BTreeMap<&'a str, f64>,
    coef: f64,
    namespace: &'a NameSpace
}

impl<'a> Monomial<'a> {
    pub fn new(
        vars: BTreeMap<&'a str, f64>,
        params: BTreeMap<&'a str, f64>,
        consts: BTreeMap<&'a str, f64>,
        coef: f64,
        namespace: &'a NameSpace
    ) -> Self {
        Self {
            vars,
            params,
            consts,
            coef,
            namespace
        }
    }

    pub fn vars(&self) -> &BTreeMap<&'a str, f64> { &self.vars }
    pub fn params(&self) -> &BTreeMap<&'a str, f64> { &self.params }
    pub fn consts(&self) -> &BTreeMap<&'a str, f64> { &self.consts }
    pub fn namespace(&self) -> &NameSpace { &self.namespace }
    /// detest whether vars, params and consts are all equal
    pub fn is_similar_term(&self, other: &Monomial) -> Result<bool, ()> {
        if eq(self.namespace(), other.namespace()) {
            Ok(
                self.vars.keys().zip(other.vars().keys()).all(|(s1, s2)| s1 == s2) &&
                self.params.keys().zip(other.params().keys()).all(|(s1, s2)| s1 == s2) &&
                self.consts.iter().zip(other.consts()).all(|(s1, s2)| s1 == s2)
            )
        } else {
            Err(())
        }
    }
    // set value to some of the params of the expression
    // 
    // notice that the value table may contants other params that is not contained in current expression
    // 
    // create a new monomial
    // pub fn set_const_v(&self, v_table: BTreeMap<&'a str, f64>) -> Self {
    //     let mut new_params = BTreeMap::new();
    //     let mut num = *self.params.get(&"1").unwrap();
    //     for c in self.params.keys() {
    //         match v_table.get(c) {
    //             Some(const_val) => {
    //                 num *= *const_val * *self.params.get(c).unwrap();
    //             }
    //             None => {
    //                 new_params.insert(*c, *self.params.get(c).unwrap());
    //             }
    //         }
    //     }
    //     new_params.insert("1", num);
    //     Self {
    //         vars: self.vars.clone(),
    //         params: new_params,
    //         consts: self.consts.clone(),
    //         namespace: self.namespace
    //     }
    // }
}

impl<'a> Add for Monomial<'a> {
    type Output = Result<Monomial<'a>, Monomial<'a>>;

    fn add(self, rhs: Self) -> Self::Output {
        match &self + &rhs {
            Ok(val) => Ok(val),
            Err(_) => Err(rhs)
        }
    }
}

impl<'a> Add for &Monomial<'a> {
    type Output = Result<Monomial<'a>, ()>;

    fn add(self, rhs: Self) -> Self::Output {
        if let Ok(res) = self.is_similar_term(&rhs) {
            if res {
                Ok(Monomial {
                    vars: self.vars.clone(),
                    params: self.params.clone(),
                    consts: self.consts.clone(),
                    coef: self.coef + rhs.coef,
                    namespace: self.namespace
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl<'a> Sub for Monomial<'a> {
    type Output = Result<Monomial<'a>, Monomial<'a>>;

    fn sub(self, rhs: Self) -> Self::Output {
        match &self - &rhs {
            Ok(val) => Ok(val),
            Err(_) => Err(rhs)
        }
    }
}

impl<'a> Sub for &Monomial<'a> {
    type Output = Result<Monomial<'a>, ()>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Ok(res) = self.is_similar_term(&rhs) {
            if res {
                Ok(Monomial {
                    vars: self.vars.clone(),
                    params: self.params.clone(),
                    consts: self.consts.clone(),
                    coef: self.coef - rhs.coef,
                    namespace: self.namespace
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl<'a> Mul for Monomial<'a> {
    type Output = Monomial<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<'a> Mul for &Monomial<'a> {
    type Output = Monomial<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut tmp_var = BTreeMap::new();
        let mut tmp_param = BTreeMap::new();
        let mut tmp_const = BTreeMap::new();

        for s in self.vars.keys() {
            if let Some(val) = rhs.vars.get(s) {
                tmp_var.insert(*s, *self.vars.get(s).unwrap() + val);
            } else {
                tmp_var.insert(*s, *self.vars.get(s).unwrap());
            }
        }
        for s in self.params.keys() {
            if let Some(val) = rhs.params.get(s) {
                tmp_param.insert(*s, *self.params.get(s).unwrap() + val);
            } else {
                tmp_param.insert(*s, *self.params.get(s).unwrap());
            }
        }
        for s in self.consts.keys() {
            if let Some(val) = rhs.consts.get(s) {
                tmp_const.insert(*s, *self.consts.get(s).unwrap() + val);
            } else {
                tmp_const.insert(*s, *self.consts.get(s).unwrap());
            }
        }
        Monomial {
            vars: tmp_var,
            params: tmp_param,
            consts: tmp_const,
            coef: self.coef * rhs.coef,
            namespace: self.namespace
        }
    }
}

impl<'a> Div for Monomial<'a> {
    type Output = Monomial<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl<'a> Div for &Monomial<'a> {
    type Output = Monomial<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        let mut tmp_var = BTreeMap::new();
        let mut tmp_param = BTreeMap::new();
        let mut tmp_const = BTreeMap::new();

        for s in self.vars.keys() {
            if let Some(val) = rhs.vars.get(s) {
                tmp_var.insert(*s, *self.vars.get(s).unwrap() - val);
            } else {
                tmp_var.insert(*s, *self.vars.get(s).unwrap());
            }
        }
        for s in self.params.keys() {
            if let Some(val) = rhs.params.get(s) {
                tmp_param.insert(*s, *self.params.get(s).unwrap() - val);
            } else {
                tmp_param.insert(*s, *self.params.get(s).unwrap());
            }
        }
        for s in self.consts.keys() {
            if let Some(val) = rhs.consts.get(s) {
                tmp_const.insert(*s, *self.consts.get(s).unwrap() - val);
            } else {
                tmp_const.insert(*s, *self.consts.get(s).unwrap());
            }
        }
        Monomial {
            vars: tmp_var,
            params: tmp_param,
            consts: tmp_const,
            coef: self.coef / rhs.coef,
            namespace: self.namespace
        }
    }
}

pub struct  MultiMonomial<'a> {
    mono: Vec<Monomial<'a>>,
}

impl<'a> MultiMonomial<'a> {
    pub fn new(mono: Vec<Monomial<'a>>) -> MultiMonomial<'a> {
        Self { mono }
    }

    pub fn mono(&self) -> &Vec<Monomial<'a>> { &self.mono }
    pub fn sort(&mut self) {
        // TODO: sort itself
    }
}

impl<'a> Add for &MultiMonomial<'a> {
    type Output = MultiMonomial<'a>;


    fn add(self, rhs: Self) -> Self::Output {
        let mut tmp: Vec<_> = vec![];
        let mut selected = vec![];
        for mono in self.mono() {
            for i in 0..rhs.mono().len() {
                match mono + &rhs.mono()[i] {
                    Ok(val) => {
                        tmp.push(val);
                        selected.push(i);
                    },
                    Err(_) => {}
                }
            }
            tmp.push(mono.clone());
        }
        for i in 0..rhs.mono().len() {
            if !selected.contains(&i) {
                tmp.push(rhs.mono()[i].clone());
            } else {}
        }
        MultiMonomial {
            mono: tmp
        }
    }
}

pub struct Expr<'a> {
    expr_tree: ExprTree<'a>,
    namespace: &'a NameSpace
}

impl<'a> Expr<'a> {
    pub fn new(namespace: &'a NameSpace) -> Self {
        Self {
            expr_tree: ExprTree::new(),
            namespace
        }
    }
}
