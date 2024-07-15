mod craphcalc;

use craphcalc::geo2d::component as cpt2d;
use craphcalc::geo2d::tool as tool2d;
use craphcalc::geo3d;
use craphcalc::geo3d::component as cpt3d;
use craphcalc::geo3d::tool as tool3d;

fn main()
{
    let p1 = cpt2d::Point{x:1.0, y:2.0};
    let p2 = cpt2d::Point{x:2.0, y:1.0};

    assert_eq!(tool2d::calc_d(&p1, &p2), (2.0_f64).sqrt());
    let v1 = geo3d::SpaceVector(2.0, 3.0, 2.0);
    let v2 = geo3d::SpaceVector(2.0, -2.0, 2.0);
    let v3 = v1.copy() * v2.copy();
    let v4 = v1.copy() % v2.copy();
    let v5 = v1.copy() * 2.0;

    println!("{}", v3);
    for v in [v1, v2, v4, v5].iter() {
        println!("{}", v);
    }
}