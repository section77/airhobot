use crate::*;
use std::iter::{self, IntoIterator};

use opencv::{
    prelude::Vector,
    types::{VectorOfPoint, VectorOfVectorOfPoint},
};

pub struct Contour(VectorOfPoint);
impl Contour {
    pub(crate) fn pack(contour: VectorOfPoint) -> Self {
        Contour(contour)
    }

    pub(crate) fn unpack(&self) -> &VectorOfPoint {
        &self.0
    }

    pub fn points(&self) -> Vec<Point> {
        self.0.iter().map(Point::pack).collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn area(&self) -> f64 {
        let closed = false;
        opencv::imgproc::contour_area(&self.unpack(), closed).expect("contour_area")
    }

    pub fn arc_length(&self, closed: bool) -> f64 {
        opencv::imgproc::arc_length(&self.unpack(), closed).expect("arc_length")
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect::pack(opencv::imgproc::bounding_rect(&self.unpack()).expect("bounding_rect"))
    }

    pub fn approx_poly_dp(&self, epsilon: f64, closed: bool) -> Self {
        let mut out = VectorOfPoint::new();
        opencv::imgproc::approx_poly_dp(self.unpack(), &mut out, epsilon, closed).expect("approx_poly_dp");
        Contour::pack(out)
    }

    pub fn center(&self) -> Point {
        let r = self.bounding_rect();
        Point::new(r.x + (r.width / 2), r.y + (r.height / 2))
    }
}

pub struct Contours(VectorOfVectorOfPoint);
impl Contours {
    pub(crate) fn pack(contours: VectorOfVectorOfPoint) -> Self {
        Contours(contours)
    }

    pub(crate) fn unpack(&self) -> &VectorOfVectorOfPoint {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
