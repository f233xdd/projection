use std::rc::Rc;
use std::cell::RefCell;

const DIGITAL: &str = "0123456789";
const SIGN: &str = "_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZαβγδεζηθικλμνξοπρστυφχψωΑΒΓΔΕΖΗΘΙΚ∧ΜΝΞΟ∏Ρ∑ΤΥΦΧΨΩ";
const OPERATION: &str = "+-*/^";
const BRACKET: &str = "()";
const COMMA: &str = ",";
const DOT: &str = ".";

struct Token {
    next: Vec<Self>,
    content: String,
    supr: Option<Rc<RefCell<Token>>>
}

impl Token {
    fn to_str(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.content);

        let mut i = self.next.len();
        while i != 0 {
            i = i - 1;
            s.push_str(&self.next[i].to_str());
        }
        s.push_str(&self.next[i].to_str());
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
    for c in SIGN.chars().into_iter() {
        if s == &c.to_string()[..] {
            return true;
        }
    }
    return false;
}

fn is_bracket(s: &str) -> bool {
    s == "()" || s == "(" || s == ")"
}

fn is_comma(s: &str) -> bool {
    s == ","
}

fn is_dot(s: &str) -> bool {
    s == "."
}


fn generate_token_tree(s: &str) -> Result<Rc<RefCell<Token>>, ()> {
    let root = Rc::new(RefCell::new(Token {next: vec![], content: "()".to_string(), supr: None}));
    let (mut slow, mut fast, mut depth): (usize, usize, i16) = (0, 0, 0);
    let mut current_node = root.clone();
    let mut before = "space";
    let mut accumulation = String::new();
    let mut s_vec = vec![];
    for c in s.to_string().chars().into_iter() {
        s_vec.push(c.to_string());
    }
    let mut have_dot;
    while fast != s_vec.len() {
        if is_digital(s_vec[fast].as_str()) {
            if before == "operation" || before == "space" || before == "left bracket" || before == "comma" {
                have_dot = false; fast += 1;
                while fast != s.len() {
                    if !is_digital(s_vec[fast].as_str()) {
                        if s_vec[fast].as_str() == DOT {
                            if have_dot {
                                panic!();
                            } else {
                                have_dot = true;
                            }
                        } else {break;}
                    } else {fast += 1;}
                }
                accumulation.clear();
                for s in s_vec[slow..fast].iter() {
                    accumulation.push_str(s.as_str());
                }
                current_node.borrow_mut().next.push(
                    Token {next: vec![], content: accumulation.clone(), supr: Some(current_node.clone())}
                );
                slow = fast;
                before = "num";
            } else {
                panic!();
            }
        } else if is_sign(s_vec[fast].as_str()) {
            if before == "operation" || before == "space" || before == "left bracket" || before == "comma" {
                fast += 1;
                while fast != s_vec.len() {
                    if !is_digital(s_vec[fast].as_str()) {
                        break;
                    } else {fast += 1;}
                }
                accumulation.clear();
                for s in s_vec[slow..fast].iter() {
                    accumulation.push_str(s.as_str());
                }
                current_node.borrow_mut().next.push(
                    Token {next: vec![], content: accumulation.clone(), supr: Some(current_node.clone())}
                );
                slow = fast;
                before = "sign";
            } else {
                panic!();
            }
        } else if is_operation(s_vec[fast].as_str()) {
            if (before == "num" || before == "sign" || before == "right bracket") 
            || ((s_vec[fast].as_str() == "+" || s_vec[fast].as_str() == "-")
            && (before == "space" || before == "left bracket" || before == "comma")) {
                fast += 1; slow = fast;
                current_node.borrow_mut().next.push(
                    Token {next: vec![], content: s_vec[fast].to_string(), supr: Some(current_node.clone())}
                );
            } else {
                panic!();
            }
        } else if s_vec[fast].as_str() == "(" {
            if before == "sign" || before == "operation" || before == "space" || before == "left bracket" || before == "comma" {
                fast += 1; slow = fast; depth += 1;
                current_node = Rc::new(RefCell::new(Token {next: vec![], content: "()".to_string(), supr: Some(current_node)}));
                before = "left bracket";
            } else {
                panic!();
            }
        } else if s_vec[fast].as_str() == ")" {
            if before == "num" || before == "sign" {
                fast += 1; slow = fast; depth += -1;
                let tmp = current_node.borrow().supr.clone().unwrap();
                current_node = tmp.clone();
                before = "right bracket";
            } else {
                panic!();
            }
        } else if s_vec[fast].as_str() == "," {
            if before == "num" || before == "sign" || before == "right bracket" {
                fast += 1; slow = fast;
                current_node.borrow_mut().next.push(
                    Token {next: vec![], content: ",".to_string(), supr: Some(current_node.clone())}
                );
                before = "comma";
            } else {
                panic!();
            }
        } else if s_vec[fast].as_str() == " " {
            fast += 1; slow = fast;
        } else {
            panic!();
        }
    }
    if depth != 0 {
        panic!();
    } else if before == "space" {
        root.borrow_mut().next.push(
            Token { next: vec![], content: "0".to_string(), supr: Some(root.clone()) }
        );
        return Ok(root);
    } else if before == "left bracket" || before == "comma"{
        panic!();
    } else {
        return Ok(root);
    }
}

#[test]
fn test_is_digital() {
    println!("{}", is_digital("12325"));
    println!("{}", is_digital("12325s"));
    println!("{}", is_digital(".112325s"));
    println!("{}", is_sign("sWXYZαβγ"));
    let t = generate_token_tree("2*3*4");
    println!("{}", t.unwrap().borrow().to_str().as_str());
}