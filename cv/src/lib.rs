use rustcv::core::{Point as RustCVPoint, Scalar};

mod utils;
pub use utils::*;
mod mat;
pub use mat::*;
mod colors;
pub use colors::*;
mod cam;
pub use cam::Cam;
mod gui;
pub use gui::GUI;
mod imageio;
pub use imageio::*;

///
pub trait ToScalar {
    fn to_scalar(&self) -> Scalar;
}

/// Point in an 2D environment
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point where {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn to_rustcv(&self) -> RustCVPoint {
        RustCVPoint { x: self.x, y: self.y }
    }

    pub fn dist(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_delta(a: f64, b: f64, delta: f64) {
        if (a - b).abs() >= delta {
            panic!(format!(
                "values not equal - left: {}, right: {}, delta: {}",
                a, b, delta
            ));
        }
    }

    #[test]
    pub fn test_dist() {
        let p1 = Point::new(3, 2);
        let p2 = Point::new(9, 7);

        assert_delta(p1.dist(&p2), 7.8, 0.1);
        assert_delta(p1.dist(&p2), p2.dist(&p1), 0.1);
    }
}

#[derive(Debug)]
pub struct CVErr {
    pub component: Component,
    pub msg: String,
}

impl CVErr {
    pub fn new(component: Component, msg: String) -> Self {
        CVErr { component, msg }
    }

    pub fn cam_err(msg: String) -> Self {
        CVErr {
            component: Component::Cam,
            msg,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Component {
    Cam,
}
