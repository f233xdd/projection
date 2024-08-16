#[allow(unused)]
use std::io::{stdin, stdout, Write};

use projection::graphcalc::{self, geo2d};


fn input(prompt: &str, s: &mut String) {
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(s).unwrap();
}

fn main() {
    let ln1 = geo2d::Line::from(1.0, 2.0, 0.0).unwrap();
    let ln2 = geo2d::Line::from(2.0, 1.0, 0.0).unwrap();
    println!("{}", ln1);
    println!("{}", ln2);
    graphcalc::test_2d::test_main();
    let l = [1, 2, 3];
    let tup = (1, 2);
}