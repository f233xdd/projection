fn main() {
    cal_things::<Test>();
}

trait Calc {
    fn cal() -> () {}
}

fn cal_things<C>()
    where C: Calc
{
    C::cal()
}

struct Test();
impl Calc for Test {
    fn cal() -> () {
        ()
    }
}