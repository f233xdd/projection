mod craphcalc;

use craphcalc::geo2d::component as cpt2d;
use craphcalc::geo2d::tool as tool2d;
use craphcalc::geo3d::component as cpt3d;
use craphcalc::geo3d::tool as tool3d;

fn main()
{
    let p1 = cpt2d::Point{x:1.0, y:2.0};
    let p2 = cpt2d::Point{x:2.0, y:1.0};

    assert_eq!(tool2d::calc_d(&p1, &p2), (2.0_f64).sqrt());
}