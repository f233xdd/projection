use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::expr::*;

pub struct Node<V> {
    next: Vec<Rc<RefCell<Self>>>,
    val: V,
    supr: Option<Rc<RefCell<Self>>>,
}

impl<V> Node<V> {
    pub fn new(next: Vec<Rc<RefCell<Self>>>, val: V, supr: Option<Rc<RefCell<Self>>>) -> Self {
        Self { next, val, supr }
    }
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
    mono: MultiMono<'a>,
    func: Vec<OneArgFnType<'a>>,
    coef: f64,
}

impl<'a> ExprCell<'a> {
    fn new(mono: MultiMono<'a>, func: Vec<OneArgFnType<'a>>, coef: f64) -> Self {
        Self { mono, func, coef }
    }
}

type ExprCellNode<'a> = Rc<RefCell<Node<ExprCell<'a>>>>;

pub struct ExprTree<'a> {
    root: ExprCellNode<'a>,
}

impl<'a> ExprTree<'a> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new(
                vec![],
                ExprCell::new(MultiMono::new(vec![]), vec![], 1.0),
                None,
            ))),
        }
    }
}

#[derive(Debug)]
pub enum MathFn {
    OneArgFn(fn(f64) -> f64),
    TwoArgFn(fn(f64, f64) -> f64),
}

impl PartialEq for MathFn {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::OneArgFn(f1) => match other {
                Self::OneArgFn(f2) => f1 == f2,
                Self::TwoArgFn(_) => false,
            },
            Self::TwoArgFn(f1) => match other {
                Self::OneArgFn(_) => false,
                Self::TwoArgFn(f2) => f1 == f2,
            },
        }
    }
}

#[derive(Debug)]
pub struct NameSpace {
    vars: Vec<String>,
    params: Vec<String>,
    consts: BTreeMap<String, f64>,
    func: Vec<MathFn>,
}

impl NameSpace {
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            params: Vec::new(),
            consts: BTreeMap::new(),
            func: Vec::new(),
        }
    }
    pub fn def_var(&mut self, v: &str) -> Result<(), ()> {
        if !self.is_defined(v) {
            self.vars.push(v.to_string());
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_param(&mut self, p: &str) -> Result<(), ()> {
        if !self.is_defined(p) {
            self.params.push(p.to_string());
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_const(&mut self, c: &str, val: f64) -> Result<(), ()> {
        if !self.is_defined(c) {
            self.consts.insert(c.to_string(), val);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn def_fn(&mut self, f: MathFn) {
        if !self.is_available(&f) {
            self.func.push(f);
        } else {
        }
    }
    pub fn is_available(&self, f: &MathFn) -> bool {
        self.func.contains(&f)
    }
    /// detect whether s is defined as a name of var/param/const
    pub fn is_defined(&self, s: &str) -> bool {
        self.is_var(s) || self.is_param(s) || self.is_const(s)
    }
    pub fn is_var(&self, s: &str) -> bool {
        self.vars.contains(&s.to_string())
    }
    pub fn is_param(&self, s: &str) -> bool {
        self.params.contains(&s.to_string())
    }
    pub fn is_const(&self, s: &str) -> bool {
        self.consts.contains_key(s)
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
