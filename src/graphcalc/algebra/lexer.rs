use std::rc::Rc;
use std::cell::RefCell;
use super::tree::Node;

pub const DIGITAL: &'static str = "0123456789";
pub const SIGN: &'static str = "_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZαβγδεζηθικλμνξοπρστυφχψωΑΒΓΔΕΖΗΘΙΚ∧ΜΝΞΟ∏Ρ∑ΤΥΦΧΨΩ";
pub const OPERATION: &'static str = "+-*/^";
pub const BRACKET: &'static str = "()";
pub const COMMA: &'static str = ",";
pub const DOT: &'static str = ".";
pub const BLANK: &'static str = " \n";

pub enum Token {
    Num(f64),
    Sign(String),
    Ops(String),
    Bracket(),
    Dot(),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Num(i) => i.to_string(),
            Self::Sign(s) => s.clone(),
            Self::Ops(s) => s.clone(),
            Self::Bracket() => String::from("()"),
            Self::Dot() => String::from(", ")
        }
    }
}

impl<V> Node<V> {
    pub fn to_string(&self) -> String 
        where V: ToString
    {
        let mut s = String::new();
        println!("{}", self.val().to_string());
        if !self.next().is_empty() {
            s.push_str("(");
            for i in 0..self.next().len() {
                s.push_str(&self.next()[i].borrow().to_string());
            }
            s.push_str(")");
        } else {
            s.push_str(&self.val().to_string());
        }
        return s;
    }
}

fn is_digital(s: &str) -> bool {
    let s = s.to_string();
    let chrs = s.chars();
    let mut res;
    for c1 in chrs.into_iter() {
        res = false;
        for c2 in DIGITAL.chars().into_iter() {
            if c1 == c2 {
                res = true;
            } else {}
        }
        if !res {
            return false;
        }
    }
    return true;
}

fn is_sign(s: &str) -> bool {
    let s = s.to_string();
    let chrs = s.chars();
    let mut res;
    for c1 in chrs.into_iter() {
        res = false;
        for c2 in SIGN.chars().into_iter() {
            if c1 == c2 {
                res = true;
            } else {}
        }
        if !res {
            return false;
        }
    }
    return true;
}

fn is_operation(s: &str) -> bool {
    for c in OPERATION.chars().into_iter() {
        if s == &c.to_string()[..] {
            return true;
        }
    }
    return false;
}

fn is_bracket(s: &str) -> bool {
    s == BRACKET || s == "(" || s == ")"
}

fn is_comma(s: &str) -> bool {
    s == COMMA
}

fn is_dot(s: &str) -> bool {
    s == DOT
}

fn is_blank(s: &str) -> bool {
    for c in BLANK.chars().into_iter() {
        if s == &c.to_string()[..] {
            return true;
        }
    }
    return false;
}

type RcRefCellTokenNode = Rc<RefCell<Node<Token>>>;

impl Node<Token> {
    pub fn from_str(s: &str)  -> Result<RcRefCellTokenNode, String> {
        Self::_from_str(s, 0, s)
    }
    fn _from_str(s: &str, i: usize, whole: &str) -> Result<Rc<RefCell<Node<Token>>>, String> {
        let root = Rc::new(RefCell::new(
            Node::new(vec![], Token::Bracket(), None)
        ));
        let (mut slow, mut fast, mut depth): (usize, usize, u16) = (0, 0, 0);
        let mut bf = "space";
        let mut s_vec = vec![];
        for c in s.to_string().chars().into_iter() {
            s_vec.push(c.to_string());
        }
        let mut have_dot;
        let mut acc = String::new();
        let mut err_msg = String::from("");
        'bottom: while fast != s_vec.len() {
            if is_digital(s_vec[fast].as_str()) {
                if bf == "operation" || bf == "space" || bf == "comma" {
                    have_dot = false; fast += 1;
                    while fast != s.len() {
                        if !is_digital(s_vec[fast].as_str()) {
                            if is_dot(s_vec[fast].as_str()) {
                                if have_dot {
                                    err_msg = format!("at {}: unexpected dot.", i+fast+1);
                                    break 'bottom;
                                } else {
                                    have_dot = true;
                                    fast += 1;
                                }
                            } else {
                                break;
                            }
                        } else { fast += 1; }
                    }
                    acc.clear();
                    for s in s_vec[slow..fast].iter() {
                        acc.push_str(s.as_str());
                    }
                    root.borrow_mut().next_mut().push(
                        Rc::new(RefCell::new(
                            Node::new(vec![], Token::Num(acc.parse::<f64>().unwrap()), Some(root.clone()))
                        ))
                    );
                    slow = fast;
                    bf = "num";
                } else {
                    err_msg = format!("at {}: {bf} can't be followed by num.", i+fast+1);
                    break;
                }
            } else if is_sign(s_vec[fast].as_str()) {
                if bf == "operation" || bf == "space" || bf == "comma" {
                    fast += 1;
                    while fast != s_vec.len() {
                        if !is_sign(s_vec[fast].as_str()) && !is_digital(s_vec[fast].as_str()) {
                            break;
                        } else { fast += 1; }
                    }
                    acc.clear();
                    for s in s_vec[slow..fast].iter() {
                        acc.push_str(s.as_str());
                    }
                    root.borrow_mut().next_mut().push(
                        Rc::new(RefCell::new(
                            Node::new(vec![], Token::Sign(acc.clone()), Some(root.clone()))
                        ))
                    );
                    slow = fast;
                    bf = "sign";
                } else {
                    err_msg = format!("at {}: {bf} can't be followed by sign.", i+fast+1);
                    break;
                }
            } else if is_operation(s_vec[fast].as_str()) {
                if (bf == "num" || bf == "sign" || bf == "right bracket") 
                || ((s_vec[fast].as_str() == "+" || s_vec[fast].as_str() == "-")
                && (bf == "space" || bf == "comma")) {
                    root.borrow_mut().next_mut().push(
                        Rc::new(RefCell::new(
                            Node::new(vec![], Token::Ops(s_vec[fast].clone()), Some(root.clone()))
                        ))
                    );
                    fast += 1; slow = fast;
                    bf = "operation"
                } else {
                    err_msg = format!("at {}: {bf} can't be followed by operation '{}'.", i+fast+1, s_vec[fast].as_str());
                    break;
                }
            } else if s_vec[fast].as_str() == "(" {
                if bf == "sign" || bf == "operation" || bf == "space" || bf == "comma" {
                    fast += 1; slow = fast; depth += 1;
                    while fast != s_vec.len() {
                        if s_vec[fast].as_str() == "(" { depth += 1; }
                        else if s_vec[fast].as_str() == ")" {
                            depth = depth - 1;
                            if depth == 0 {
                                acc.clear();
                                for s in s_vec[slow..fast].iter() {
                                    acc.push_str(s.as_str());
                                }
                                root.borrow_mut().next_mut().push(
                                    Self::_from_str(acc.as_str(), slow+i, whole).unwrap()
                                );
                                break
                            } else {}
                        }
                        else {}
                        fast += 1;
                    }
                    if depth != 0 { 
                        err_msg = format!("at {}: bracket is not closed.", i+fast+1);
                        break;
                    }
                    else { fast += 1; slow = fast; bf = "right bracket"; }
                } else {
                    err_msg = format!("at {}: {bf} can't be followed by left bracket.", i+fast+1);
                    break;
                }
            } else if is_comma(s_vec[fast].as_str()) {
                if bf == "num" || bf == "sign" || bf == "right bracket" {
                    fast += 1; slow = fast;
                    root.borrow_mut().next_mut().push(
                        Rc::new(RefCell::new(
                            Node::new(vec![], Token::Dot(), Some(root.clone()))
                        ))
                    );
                    bf = "comma";
                } else {
                    err_msg = format!("at {}: {bf} can't be followed by comma.", i+fast+1);
                    break;
                }
            } else if is_blank(s_vec[fast].as_str()) {
                fast += 1; slow = fast;
            } else {
                if s_vec[fast].as_str() == ")" {
                    err_msg = format!("at {}: bracket is not closed.", i+fast+1);
                } else {
                    err_msg = format!("at {}: invalid token.", i+fast+1);
                }
                break;
            }
        }
        if err_msg != "" {
            return Err(err_msg);
        } else if root.borrow().next().is_empty() {
            root.borrow_mut().next_mut().push(
                Rc::new(RefCell::new(
                    Node::new(vec![], Token::Num(0.0), Some(root.clone()))
                ))
            );
            return Ok(root);
        } else if bf == "operation" || bf == "comma" {
            Err(format!("at {}: expr can't end with {bf}.", i+fast+1))
        } else {
            return Ok(root);
        }
    }
}

#[test]
fn test_parse() {
    // println!("{}", is_digital("12325"));
    // println!("{}", is_digital("12325s"));
    // println!("{}", is_digital(".112325s"));
    // println!("{}", is_sign("sWXYZαβγ"));
    let t = Node::from_str(
        "abs(2*lg(pow(cos(abs(a*x*(2.012)^(-a))), 5)-λ)*x^2+3*tan(sh(λ*y^5*z+x)+z))-
        3*pow(sin(-abs(x^a-φ)*y)-b^3*c^(-5)+10*ln(sec(x*z^(e^x)-2^c)), a/b)*lg(abs(x))"
    );
    println!("{}", t.unwrap().borrow().to_string().as_str());
}
