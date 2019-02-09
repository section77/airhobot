use rustcv::core::Point as RustCVPoint;
use std::iter;
mod utils;
pub use utils::*;
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
    fn to_rustcv(&self) -> rustcv::core::CvType {
        use CVType::*;
        match &self {
            CV8UC1 => rustcv::core::CvType::Cv8UC1,
            CV8SC1 => rustcv::core::CvType::Cv8SC1,
            CV8UC3 => rustcv::core::CvType::Cv8UC3,
            CV8SC3 => rustcv::core::CvType::Cv8SC3,
        }
    }
}

/// Point in an 2D environment
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point where {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn from_rustcv(p: RustCVPoint) -> Self {
        Self::new(p.x, p.y)
    }

    fn to_rustcv(&self) -> RustCVPoint {
        RustCVPoint { x: self.x, y: self.y }
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
    fn from_rustcv(rect: rustcv::core::Rect) -> Self {
        Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }

    fn to_rustcv(&self) -> rustcv::core::Rect {
        rustcv::core::Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contour(Vec<Point>);
impl Contour {
    fn from_rustcv(contour: rustcv::core::Contour) -> Self {
        Contour(
            // (0..(contour.length as isize))
            //     .map(|i| Point::from_rustcv(unsafe { *contour.points.offset(i) }))
            //     .collect(),
            unsafe { std::slice::from_raw_parts(contour.points, contour.length as usize) }
                .iter()
                .map(|p| Point::from_rustcv(*p))
                .collect(),
        )
    }

    fn to_rustcv(&self) -> rustcv::core::Points {
        let mut points: Vec<rustcv::core::Point> = self.0.iter().map(|p| p.to_rustcv()).collect();

        points.shrink_to_fit();
        let ptr = points.as_mut_ptr();
        std::mem::forget(points);

        rustcv::core::Points {
            points: ptr, // points.as_mut_ptr(),
            length: self.0.len() as i32,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn area(&self) -> f64 {
        rustcv::imgproc::contour_area(self.to_rustcv())
    }

    pub fn arc_length(&self, closed: bool) -> f64 {
        let mut points: Vec<rustcv::core::Point> = self.0.iter().map(|p| p.to_rustcv()).collect();
        rustcv::imgproc::arc_length(&mut points, closed)
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect::from_rustcv(rustcv::imgproc::bounding_rect(self.to_rustcv()))
    }

    pub fn approx_poly_dp(&self, epsilon: f64, closed: bool) -> Self {
        Contour::from_rustcv(rustcv::imgproc::approx_poly_dp(self.to_rustcv(), epsilon, closed))
    }

    pub fn center(&self) -> Point {
        let r = self.bounding_rect();
        Point::new(r.x + (r.width / 2), r.y + (r.height / 2))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Contours(Vec<Contour>);
impl Contours {
    pub fn new() -> Self {
        Contours(Vec::new())
    }

    pub fn add(&mut self, contour: Contour) {
        self.0.push(contour);
    }

    fn from_rustcv(contours: rustcv::core::Contours) -> Self {
        Contours(
            unsafe { std::slice::from_raw_parts(contours.contours, contours.length as usize) }
                .iter()
                .map(|p| Contour::from_rustcv(*p))
                .collect(),
        )
    }

    fn to_rustcv(&self) -> rustcv::core::Contours {
        let mut contours: Vec<rustcv::core::Contour> = self.0.iter().map(|p| p.to_rustcv()).collect();

        contours.shrink_to_fit();
        let ptr = contours.as_mut_ptr();
        std::mem::forget(contours);

        rustcv::core::Contours {
            contours: ptr, //  contours.as_mut_ptr(),
            length: self.0.len() as i32,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Contour> {
        self.0.iter()
    }

    pub fn into_iter(&self) -> std::vec::IntoIter<Contour> {
        self.0.clone().into_iter()
    }
}

impl iter::FromIterator<Contour> for Contours {
    fn from_iter<I: IntoIterator<Item = Contour>>(iter: I) -> Self {
        let mut contours = Contours::new();
        for c in iter {
            contours.add(c);
        }
        contours
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
