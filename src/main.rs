use projection::{graphcalc::{test_2d, test_3d}, p, ln, pn, vector};

fn main() {
    test_2d::test_main();
    test_3d::test_main();
}

// use std::io::{stdin, stdout, Write};
// fn input(prompt: &str, s: &mut String) {
//     print!("{}", prompt);
//     stdout().flush().unwrap();
//     stdin().read_line(s).unwrap();
// }