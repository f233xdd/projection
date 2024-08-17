#[macro_export]
macro_rules! p {
    // 2D
    ($x:expr, $y:expr) => {
        $crate::graphcalc::geo2d::Point::new($x as f64, $y as f64)
    };
    // 3D
    ($x:expr, $y:expr, $z:expr) => {
        $crate::graphcalc::geo3d::Point::new($x as f64, $y as f64, $z as f64)
    };
}

#[macro_export]
macro_rules! ln {
    // 2D
    // support five line function form:
    //     standard form
    //     slope-intercept form
    //     point-slope form
    //     two-point form
    //     intercept form
    (k1=$k1:expr, k2=$k2:expr, b=$b:expr) => { // standard form
        $crate::graphcalc::geo2d::Line::new($k1 as f64, $k2 as f64, $b as f64)
    };
    (k=$k:expr, b=$b:expr) => { // slope-intercept form
        $crate::graphcalc::geo2d::Line::new(-$k as f64, 1.0, $b as f64).unwrap()
    };
    (($x0:expr, $y0:expr), k=$k:expr) => { // point-slope form
        $crate::graphcalc::geo2d::Line::new($k as f64, -1.0, ($k as f64)*($x0 as f64)-($y0 as f64))
    };
    (($x1:expr, $y1:expr), ($x2:expr, $y2:expr)) => { // two-point form
        $crate::graphcalc::geo2d::Line::new(($y2 as f64)-($y1 as f64), ($x1 as f64)-($x2 as f64), ($x1 as f64)*($y2 as f64)-($x2 as f64)*($y1 as f64))
    };
    (b1=$b1:expr, b2=$b2:expr) => { // intercept form
        if $b1 != 0.0 && $b2 != 0.0 {
            $crate::graphcalc::geo2d::Line::new(1.0/($b1 as f64), 1.0/($b2 as f64), 1.0)
        } else {
            Err(())
        }
    };
    // 3D
    (($x1:expr, $y1:expr, $z1:expr), ($x2:expr, $y2:expr, $z2:expr)) => {
        $crate::graphcalc::geo3d::Line::from(
            &$crate::graphcalc::geo3d::Point::new($x1 as f64, $y1 as f64, $z1 as f64),
            &$crate::graphcalc::geo3d::Point::new($x2 as f64, $y2 as f64, $z2 as f64)
        )
    };
    (($k11:expr, $k12:expr, $b1:expr), ($k21:expr, $k22:expr, $b2:expr)) => {
        $crate::graphcalc::geo3d::Line::new(k11 as f64, k12 as f64, b1 as f64, k21 as f64, k22 as f64, b2 as f64)
    }
}

#[macro_export]
macro_rules! pn {
    (
        ($x1:expr, $y1:expr, $z1:expr), 
        ($x2:expr, $y2:expr, $z2:expr), 
        ($x3:expr, $y3:expr, $z3:expr)
    ) => {
        $crate::graphcalc::geo3d::Plane::from(
            &$crate::graphcalc::geo3d::Point::new($x1 as f64, $y1 as f64, $z1 as f64),
            &$crate::graphcalc::geo3d::Point::new($x2 as f64, $y2 as f64, $z2 as f64),
            &$crate::graphcalc::geo3d::Point::new($x3 as f64, $y3 as f64, $z3 as f64)
        )
    };
    ($k1:expr, $k2:expr, $k3:expr, $b:expr) => {
        $crate::graphcalc::geo3d::Plane::new($k1 as f64, $k2 as f64, $k3 as f64, $b as f64)
    }
}

#[macro_export]
macro_rules! vector {
    // 2D
    ($x:expr, $y:expr) => {
        $crate::graphcalc::geo2d::PlaneVector::new($x as f64, $y as f64)
    };
    // 3D
    ($x:expr, $y:expr, $z:expr) => {
        $crate::graphcalc::geo3d::SpaceVector::new($x as f64, $y as f64, $z as f64)
    }
}
