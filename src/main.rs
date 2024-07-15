mod craphcalc;

use craphcalc::geo2d;
use craphcalc::geo3d;

fn main()
{
    let p1 = geo2d::Point{x:1.0, y:2.0};
    let p2 = geo2d::Point{x:2.0, y:1.0};

    assert_eq!(geo2d::calc_d(&p1, &p2), (2.0_f64).sqrt());
    let v1 = geo3d::SpaceVector(2.0, 3.0, 2.0);
    let v2 = geo3d::SpaceVector(2.0, -2.0, 2.0);
    let v3 = &v1 * &v2;
    let v4 = &v1 % &v2;
    let v5 = &v1 * 2.0;

    println!("{}", v3);
    for v in [v1, v2, v4, v5].iter() {
        println!("{}", v);
    }
}
