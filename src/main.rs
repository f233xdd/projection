use std::fmt::Display;

struct Point<T1: Display, T2: Display, T3: Display> (T1, T2, T3);

impl<T1: Display, T2: Display, T3: Display> Point<T1, T2, T3> {
    fn format(&self) -> String {
        format!("({}, {}, {})", self.0, self.1, self.2)
    }
}

fn main()
{
    let p: Point<f64, i32, f64> = Point(1.0, 2, 0.2);
    let s: String = p.format();
    println!("{}", s);
}
