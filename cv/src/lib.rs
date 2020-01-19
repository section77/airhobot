use std::iter;
use std::iter::IntoIterator;
use std::slice::Iter;
mod mat;
pub use mat::*;
mod colors;
pub use colors::*;
mod videoio;
pub use videoio::*;
mod gui;
pub use gui::GUI;
mod imageio;
pub use imageio::*;
mod err;
pub use err::*;
use opencv::{
    core::Point as OpencvPoint,
    prelude::Vector,
    types::{VectorOfPoint, VectorOfVectorOfPoint},
};

type Result<T> = std::result::Result<T, CVErr>;

#[derive(Debug, Clone, Copy)]
pub enum CVType {
    /// 8 bit unsigned, single channel
    CV8UC1 = 0,
    /// 8 bit signed, single channel
    CV8SC1 = 1,
    /// 8 bit unsigned, three channels
    CV8UC3 = 16,
    /// 8 bit signed, three channel
    CV8SC3 = 17,
}

impl CVType {
    fn unpack(&self) -> i32 {
        *self as i32
    }
}

/// Point in an 2D environment
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn pack(p: opencv::core::Point) -> Self {
        Self::new(p.x, p.y)
    }

    fn unpack(&self) -> opencv::core::Point {
        opencv::core::Point_ { x: self.x, y: self.y }
    }

    pub fn dist(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn pack(rect: opencv::core::Rect) -> Self {
        Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }

    fn unpack(&self) -> opencv::core::Rect {
        opencv::core::Rect_ {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

pub struct Contour(VectorOfPoint);
impl Contour {
    fn pack(contour: VectorOfPoint) -> Self {
        Contour(contour)
    }

    fn unpack(&self) -> &VectorOfPoint {
        &self.0
    }

    pub fn points(&self) -> Vec<Point> {
        self.0.iter().map(Point::pack).collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn area(&self) -> Result<f64> {
        let closed = false;
        Ok(opencv::imgproc::contour_area(&self.unpack(), closed)?)
    }

    pub fn arc_length(&self) -> Result<f64> {
        let closed = false;
        Ok(opencv::imgproc::arc_length(&self.unpack(), closed)?)
    }

    pub fn bounding_rect(&self) -> Result<Rect> {
        Ok(Rect::pack(opencv::imgproc::bounding_rect(&self.unpack())?))
    }

    pub fn approx_poly_dp(&self, epsilon: f64, closed: bool) -> Result<Self> {
        let mut out = VectorOfPoint::new();
        opencv::imgproc::approx_poly_dp(self.unpack(), &mut out, epsilon, closed)?;
        Ok(Contour::pack(out))
    }

    pub fn center(&self) -> Result<Point> {
        let r = self.bounding_rect()?;
        Ok(Point::new(r.x + (r.width / 2), r.y + (r.height / 2)))
    }
}

pub struct Contours(VectorOfVectorOfPoint);
impl Contours {
    fn pack(contours: VectorOfVectorOfPoint) -> Self {
        Contours(contours)
    }

    fn unpack(&self) -> &VectorOfVectorOfPoint {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = Contour> + '_ {
        self.0.iter().into_iter().map(|v| Contour::pack(v))
    }
}

impl iter::FromIterator<Contour> for Contours {
    fn from_iter<I: IntoIterator<Item = Contour>>(iter: I) -> Self {
        let iter_points = iter.into_iter().map(|c| c.0);
        let vec_points: VectorOfVectorOfPoint = Vector::from_iter(iter_points);
        Contours::pack(vec_points)
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
        let p1 = Point::pack(3, 2);
        let p2 = Point::pack(9, 7);

        assert_delta(p1.dist(&p2), 7.8, 0.1);
        assert_delta(p1.dist(&p2), p2.dist(&p1), 0.1);
    }
}
