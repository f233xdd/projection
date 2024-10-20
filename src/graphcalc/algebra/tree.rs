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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct NameSpace {
    var: Vec<&'static str>,
    param: Vec<&'static str>,
    func: Vec<MathFn>,
}

impl NameSpace {
    pub fn new() -> Self {
        Self {
            var: Vec::new(),
            param: Vec::new(),
            func: Vec::new()
        }
    }
    pub fn def_var(&mut self, v: &'static str) -> Result<(), ()> {
        if !self.is_defined(v) {
            self.var.push(v);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_param(&mut self, p: &'static str) -> Result<(), ()> {
        if !self.is_defined(p) {
            self.param.push(p);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_fn(&mut self, f: MathFn) {
        if !self.is_available(&f) {
            self.func.push(f);
        } else {}
    }
    pub fn is_available(&self, f: &MathFn) -> bool {
        self.func.contains(&f)
    }
    pub fn is_defined(&self, s: &str) -> bool {
        self.is_var(s) || self.is_param(s)
    }
    pub fn is_var(&self, s: &str) -> bool {
        self.var.contains(&s)
    }
    pub fn is_param(&self, s: &str) -> bool {
        self.param.contains(&s)
    }
}

impl Into<MathFn> for fn(f64) -> f64 {
    fn into(self) -> MathFn {
        MathFn::OneArgFn(self)
    }
}

impl Into<MathFn> for fn(f64, f64) -> f64 {
    fn into(self) -> MathFn {
        MathFn::TwoArgFn(self)
    }
}