use super::expr::*;

enum OneArgFnType<'a> {
    InitFn(fn(f64, f64) -> f64, Expr<'a>),
    Fn(fn(f64) -> f64),
}

enum Node<'a> {
    // $coef * $($func)* ( $($mono)* + $($next)* )
    Add{
        mono: Vec<Monomial<'a>>,
        func: Vec<OneArgFnType<'a>>,
        coef: f64,
        next: Vec<Box<Node<'a>>>,
        supr: Option<&'a Node<'a>>,
    },
    // $coef * $($func)* ( $mono * $($next)* )
    Mul{
        mono: Monomial<'a>,
        func: Vec<OneArgFnType<'a>>,
        coef: f64,
        next: Vec<Box<Node<'a>>>,
        supr: Option<&'a Node<'a>>,
    },
}

pub struct ExprTree<'a> {
    root: Node<'a>
}

impl<'a> ExprTree<'a> {
    pub fn new() -> Self {
        Self {
            root: Node::Add { 
                mono: Vec::new(),
                func: Vec::new(),
                coef: 0.0,
                next: Vec::new(),
                supr: None,
            }
        }
    }
}

pub enum MathFn {
    OneArgFn(fn(f64)->f64),
    TwoArgFn(fn(f64, f64)->f64)
}

impl PartialEq for MathFn {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::OneArgFn(f1) => {
                match other {
                    Self::OneArgFn(f2) => {
                        if f1 == f2 {true} else {false}
                    },
                    Self::TwoArgFn(_) => {
                        false
                    }
                }
            },
            Self::TwoArgFn(f1) => {
                match other {
                    Self::OneArgFn(_) => {
                        false
                    },
                    Self::TwoArgFn(f2) => {
                        if f1 == f2 {true} else {false}
                    }
                }
            }
        }
    }
}

pub struct NameSpace {
    variable: Vec<&'static str>,
    constant: Vec<&'static str>,
    avail_func: Vec<MathFn>,
}

impl NameSpace {
    pub fn new() -> Self {
        Self {
            variable: Vec::new(),
            constant: Vec::new(),
            avail_func: Vec::new()
        }
    }
    pub fn def_var(&mut self, v: &'static str) -> Result<(), ()> {
        if !self.is_defined(v) {
            self.variable.push(v);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_const(&mut self, c: &'static str) -> Result<(), ()> {
        if !self.is_defined(c) {
            self.constant.push(c);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_fn(&mut self, f: MathFn) {
        if !self.is_available(&f) {
            self.avail_func.push(f);
        } else {}
    }
    pub fn is_available(&self, f: &MathFn) -> bool {
        self.avail_func.contains(&f)
    }
    pub fn is_defined(&self, s: &str) -> bool {
        self.is_var(s) || self.is_const(s)
    }
    pub fn is_var(&self, s: &str) -> bool {
        self.variable.contains(&s)
    }
    pub fn is_const(&self, s: &str) -> bool {
        self.constant.contains(&s)
    }
}