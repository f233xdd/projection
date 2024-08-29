#[macro_export]
macro_rules! p {
    // 2D
    ($x:expr, $y:expr) => {
        $crate::graphcalc::geo2d::Point::new($x, $y)
    };
    // 3D
    ($x:expr, $y:expr, $z:expr) => {
        $crate::graphcalc::geo3d::Point::new($x, $y, $z)
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
    (k1: $k1:expr, k2: $k2:expr, b: $b:expr) => { // standard form
        $crate::graphcalc::geo2d::Line::new($k1, $k2, $b)
    };
    (k: $k:expr, b: $b:expr) => { // slope-intercept form
        $crate::graphcalc::geo2d::Line::new(-$k, 1.0, $b).unwrap()
    };
    (($x0:expr, $y0:expr), k: $k:expr) => { // point-slope form
        $crate::graphcalc::geo2d::Line::new($k, -1.0, $k*$x0-$y0)
    };
    (($x1:expr, $y1:expr), ($x2:expr, $y2:expr)) => { // two-point form
        $crate::graphcalc::geo2d::Line::new($y2-$y1, $x1-$x2, $x1*$y2-$x2*$y1)
    };
    (b1: $b1:expr, b2: $b2:expr) => { // intercept form
        if $b1 != 0.0 && $b2 != 0.0 {
            $crate::graphcalc::geo2d::Line::new(1.0/$b1, 1.0/$b2, 1.0)
        } else {
            Err(())
        }
    };
    // 3D
    (($x1:expr, $y1:expr, $z1:expr), ($x2:expr, $y2:expr, $z2:expr)) => {
        $crate::graphcalc::geo3d::Line::from(
            &$crate::graphcalc::geo3d::Point::new($x1, $y1, $z1),
            &$crate::graphcalc::geo3d::Point::new($x2, $y2, $z2)
        )
    };
    (
        (k11: $k11:expr, k12: $k12:expr, k13: $k13:expr, b1: $b1:expr), 
        (k21: $k21:expr, k22: $k22:expr, k23: $k23:expr, b2: $b2:expr)
    ) => {
        $crate::graphcalc::geo3d::Line::new($k11, $k12, $k13, $b1, $k21, $k22, $k23, $b2)
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
            &$crate::graphcalc::geo3d::Point::new($x1, $y1, $z1),
            &$crate::graphcalc::geo3d::Point::new($x2, $y2, $z2),
            &$crate::graphcalc::geo3d::Point::new($x3, $y3, $z3)
        )
    };
    (k1: $k1:expr, k2: $k2:expr, k3: $k3:expr, b: $b:expr) => {
        $crate::graphcalc::geo3d::Plane::new($k1, $k2, $k3, $b)
    }
}

#[macro_export]
macro_rules! vector {
    // 2D
    ($x:expr, $y:expr) => {
        $crate::graphcalc::geo2d::PlaneVector::new($x, $y)
    };
    // 3D
    ($x:expr, $y:expr, $z:expr) => {
        $crate::graphcalc::geo3d::SpaceVector::new($x, $y, $z)
    }
}
