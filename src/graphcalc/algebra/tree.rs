use std::rc::Rc;
use std::cell::RefCell;

use super::expr::*;

pub struct Node<V> {
    next: Vec<Rc<RefCell<Self>>>,
    val: V,
    supr: Option<Rc<RefCell<Self>>>
}

impl<V> Node<V> {
    pub fn new(    
        next: Vec<Rc<RefCell<Self>>>,
        val: V,
        supr: Option<Rc<RefCell<Self>>>
    ) -> Self { Self { next, val, supr } }
    pub fn val(&self) -> &V {
        &self.val
    }
    pub fn val_mut(&mut self) -> &mut V {
        &mut self.val
    }
    pub fn next(&self) -> &Vec<Rc<RefCell<Self>>> {
        &self.next
    }
    pub fn next_mut(&mut self) -> &mut Vec<Rc<RefCell<Self>>> {
        &mut self.next
    }
    pub fn supr(&self) -> &Option<Rc<RefCell<Self>>> {
        &self.supr
    }
    pub fn supr_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>> {
        &mut self.supr
    }
}

enum OneArgFnType<'a> {
    InitFn(fn(f64, f64) -> f64, Expr<'a>),
    Fn(fn(f64) -> f64),
}

struct ExprCell<'a> {
    mono: Vec<Monomial<'a>>,
    func: Vec<OneArgFnType<'a>>,
    coef: f64,
}

impl<'a> ExprCell<'a> {
    fn new(
        mono: Vec<Monomial<'a>>,
        func: Vec<OneArgFnType<'a>>,
        coef: f64,
    ) -> Self { Self { mono, func, coef } }
}

type ExprCellNode<'a> = Rc<RefCell<Node<ExprCell<'a>>>>;

pub struct ExprTree<'a> {
    root: ExprCellNode<'a>
}

impl<'a> ExprTree<'a> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new(
                vec![], 
                ExprCell::new(vec![], vec![], 1.0),
                None
            )))
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