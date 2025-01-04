use super::lexer::SIGN;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use std::ptr::eq;

use super::tree::*;

#[derive(Clone)]
pub struct Monomial<'a> {
    vars: BTreeMap<&'a str, f64>,
    params: BTreeMap<&'a str, f64>,
    consts: BTreeMap<&'a str, f64>,
    coef: f64,
    namespace: &'a NameSpace,
}

impl<'a> Monomial<'a> {
    pub fn new(
        vars: BTreeMap<&'a str, f64>,
        params: BTreeMap<&'a str, f64>,
        consts: BTreeMap<&'a str, f64>,
        coef: f64,
        namespace: &'a NameSpace,
    ) -> Result<
        Self,
        (
            BTreeMap<&'a str, f64>,
            BTreeMap<&'a str, f64>,
            BTreeMap<&'a str, f64>,
        ),
    > {
        if [&vars, &params, &consts]
            .into_iter()
            .all(|map| map.keys().all(|key| namespace.is_defined(key)))
        {
            Ok(Self {
                vars,
                params,
                consts,
                coef,
                namespace,
            })
        } else {
            Err((vars, params, consts))
        }
    }

    pub fn vars(&self) -> &BTreeMap<&'a str, f64> {
        &self.vars
    }
    pub fn params(&self) -> &BTreeMap<&'a str, f64> {
        &self.params
    }
    pub fn consts(&self) -> &BTreeMap<&'a str, f64> {
        &self.consts
    }
    pub fn coef(&self) -> f64 {
        self.coef
    }
    pub fn namespace(&self) -> &NameSpace {
        &self.namespace
    }
    /// detest whether vars and params are all the same
    ///
    /// return Err if namespace is not the same
    pub fn is_similar_term(&self, other: &Monomial) -> Result<bool, ()> {
        if eq(self.namespace(), other.namespace()) {
            Ok(self
                .vars
                .keys()
                .zip(other.vars().keys())
                .all(|(s1, s2)| s1 == s2)
                && self
                    .params
                    .keys()
                    .zip(other.params().keys())
                    .all(|(s1, s2)| s1 == s2))
        } else {
            Err(())
        }
    }
    pub fn sort(&mut self) {
        match self.coef {
            f64::INFINITY | f64::NEG_INFINITY => {
                for map in [&mut self.vars, &mut self.params, &mut self.consts].into_iter() {
                    map.clear();
                }
            }
            _ if self.coef.is_nan() || self.is_zero() => {
                for map in [&mut self.vars, &mut self.params, &mut self.consts].into_iter() {
                    map.clear();
                }
            }
            _ => {}
        }
    }
    pub fn is_zero(&self) -> bool {
        self.coef == 0.0
    }
    // set value to some of the params of the expression
    //
    // notice that the value table may constants other params that is not contained in current expression
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
    type Output = Result<Monomial<'a>, (Monomial<'a>, Monomial<'a>)>;

    fn add(self, rhs: Self) -> Self::Output {
        match &self + &rhs {
            Ok(val) => Ok(val),
            _ => Err((self, rhs)),
        }
    }
}

impl<'a> Add for &Monomial<'a> {
    type Output = Result<Monomial<'a>, ()>;

    fn add(self, rhs: Self) -> Self::Output {
        match self.is_similar_term(&rhs) {
            Ok(res) if self.coef() == rhs.coef() => {
                if res {
                    let mut tmp = Monomial {
                        vars: self.vars.clone(),
                        params: self.params.clone(),
                        consts: self.consts.clone(),
                        coef: self.coef + rhs.coef,
                        namespace: &self.namespace,
                    };
                    tmp.sort();
                    Ok(tmp)
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

impl<'a> Sub for Monomial<'a> {
    type Output = Result<Monomial<'a>, (Monomial<'a>, Monomial<'a>)>;

    fn sub(self, rhs: Self) -> Self::Output {
        match &self - &rhs {
            Ok(val) => Ok(val),
            _ => Err((self, rhs)),
        }
    }
}

impl<'a> Sub for &Monomial<'a> {
    type Output = Result<Monomial<'a>, ()>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self.is_similar_term(&rhs) {
            Ok(res) if self.coef() == rhs.coef() => {
                if res {
                    let mut tmp = Monomial {
                        vars: self.vars.clone(),
                        params: self.params.clone(),
                        consts: self.consts.clone(),
                        coef: self.coef - rhs.coef,
                        namespace: &self.namespace,
                    };
                    tmp.sort();
                    Ok(tmp)
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

impl<'a> Mul for Monomial<'a> {
    type Output = Result<Monomial<'a>, (Monomial<'a>, Monomial<'a>)>;

    fn mul(self, rhs: Self) -> Self::Output {
        match &self * &rhs {
            Ok(val) => Ok(val),
            _ => Err((self, rhs)),
        }
    }
}

impl<'a> Mul for &Monomial<'a> {
    type Output = Result<Monomial<'a>, ()>;

    fn mul(self, rhs: Self) -> Self::Output {
        if eq(self.namespace(), rhs.namespace()) {
            let mut tmp = [
                (self.vars(), rhs.vars()),
                (self.params(), rhs.params()),
                (self.consts(), rhs.consts()),
            ]
            .into_iter()
            .map(|(map1, map2)| {
                map1.keys()
                    .chain(map2.keys().filter(|&&key| !map1.contains_key(key)))
                    .map(|&key| match (map1.get(key), map2.get(key)) {
                        (Some(&val1), Some(&val2)) => (key, val1 + val2),
                        (Some(&val1), None) => (key, val1),
                        (None, Some(&val2)) => (key, val2),
                        _ => ("ERROR", 1.0), // it's impossible to reach here
                    })
                    .collect::<BTreeMap<&str, f64>>()
            });
            let mut tmp = Monomial {
                vars: tmp.next().unwrap(),
                params: tmp.next().unwrap(),
                consts: tmp.next().unwrap(),
                coef: self.coef * rhs.coef,
                namespace: self.namespace,
            };
            tmp.sort();
            Ok(tmp)
        } else {
            Err(())
        }
    }
}

impl<'a> Div for Monomial<'a> {
    type Output = Result<Monomial<'a>, (Monomial<'a>, Monomial<'a>)>;

    fn div(self, rhs: Self) -> Self::Output {
        match &self / &rhs {
            Ok(val) => Ok(val),
            _ => Err((self, rhs)),
        }
    }
}

impl<'a> Div for &Monomial<'a> {
    type Output = Result<Monomial<'a>, ()>;

    fn div(self, rhs: Self) -> Self::Output {
        if eq(self.namespace(), rhs.namespace()) {
            let mut tmp = [
                (self.vars(), rhs.vars()),
                (self.params(), rhs.params()),
                (self.consts(), rhs.consts()),
            ]
            .into_iter()
            .map(|(map1, map2)| {
                map1.keys()
                    .chain(map2.keys().filter(|&&key| !map1.contains_key(key)))
                    .map(|&key| match (map1.get(key), map2.get(key)) {
                        (Some(&val1), Some(&val2)) => (key, val1 - val2),
                        (Some(&val1), None) => (key, val1),
                        (None, Some(&val2)) => (key, val2),
                        _ => ("ERROR", 1.0),
                    })
                    .collect::<BTreeMap<&str, f64>>()
            });
            let mut tmp = Monomial {
                vars: tmp.next().unwrap(),
                params: tmp.next().unwrap(),
                consts: tmp.next().unwrap(),
                coef: self.coef / rhs.coef,
                namespace: self.namespace,
            };
            tmp.sort();
            Ok(tmp)
        } else {
            Err(())
        }
    }
}

impl<'a> Display for Monomial<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.coef {
                0.0 => "0.0".to_string(),
                f64::INFINITY => "∞".to_string(),
                f64::NEG_INFINITY => "-∞".to_string(),
                _ if self.coef.is_nan() => "NaN".to_string(),
                _ => {
                    let mut buf = if self.coef > 0.0 {
                        "+".to_string()
                    } else {
                        "-".to_string()
                    };
                    buf.push_str(
                        [self.consts(), self.params(), self.vars()]
                            .into_iter()
                            .map(|map| {
                                SIGN.chars()
                                    .filter_map(|chr| match map.get(chr.to_string().as_str()) {
                                        Some(val) => Some(format!("*{}^{val}", chr)),
                                        None => None,
                                    })
                                    .collect::<String>()
                            })
                            .collect::<String>()
                            .as_str(),
                    );
                    buf
                }
            }
        )
    }
}

pub struct MultiMono<'a> {
    mono: Vec<Monomial<'a>>,
}

impl<'a> MultiMono<'a> {
    pub fn new(mono: Vec<Monomial<'a>>) -> MultiMono<'a> {
        Self { mono }
    }

    pub fn mono(&self) -> &Vec<Monomial<'a>> {
        &self.mono
    }
    pub fn sort(&mut self) {
        // TODO: sort itself
        // num > const > param > var
        // a-z > A-Z > α-ω > Α-Ω
    }
}

impl<'a> Display for MultiMono<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.mono()
                .iter()
                .map(|mono| mono.to_string())
                .collect::<String>()
        )
    }
}

pub struct Expr<'a> {
    expr_tree: ExprTree<'a>,
    namespace: &'a NameSpace,
}

impl<'a> Expr<'a> {
    pub fn new(namespace: &'a NameSpace) -> Self {
        Self {
            expr_tree: ExprTree::new(),
            namespace,
        }
    }
}

fn need_sort(mono1: &Monomial, mono2: &Monomial) -> bool {
    true
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::define;

    use super::{Monomial, NameSpace};

    #[test]
    fn test_monomial() {
        let mut np = NameSpace::new();
        define! {
            using namespace np;
            let a, b, c: param;
            let x, y, z: var;
        }
        let mono_1 = Monomial::new(BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), 0.0, &np);
        println!("{}", mono_1.unwrap());
    }
    #[test]
    fn test_multi_mono() {}
    #[test]
    fn test_expr() {}
}
