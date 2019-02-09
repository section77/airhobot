use log::warn;
use rustcv::core::in_range_with_scalar;
use rustcv::core::Mat as RustCVMat;
use rustcv::imgproc::*;

use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Gray;

#[derive(Debug)]
pub struct Mat<ColorSpace> {
    inner: RustCVMat,
    n_rows: i32,
    n_cols: i32,
    _type: PhantomData<ColorSpace>,
}

impl<ColorSpace> Mat<ColorSpace> where {
    pub fn new<C: ToScalar>(rows: i32, cols: i32, cv_type: &CVType, background: C) -> Mat<ColorSpace> {
        Mat::from_rustcv(RustCVMat::new_with_size_from_scalar(
            rows,
            cols,
            cv_type.to_rustcv(),
            background.to_scalar(),
        ))
    }

    pub fn from_rustcv(inner: RustCVMat) -> Mat<ColorSpace> {
        let n_rows = inner.rows();
        let n_cols = inner.cols();
        Mat {
            inner,
            n_rows,
            n_cols,
            _type: PhantomData,
        }
    }

    pub fn to_rustcv(&self) -> &RustCVMat {
        &self.inner
    }

    pub fn n_rows(&self) -> i32 {
        self.n_rows
    }

    pub fn n_cols(&self) -> i32 {
        self.n_cols
    }

    pub fn draw_contours<T: ToScalar>(&mut self, contours: &Contours, color: T, thickness: i32) {
        let contours = contours.to_rustcv();
        for idx in 0..contours.length {
            rustcv::imgproc::draw_contours(&mut self.inner, contours, idx as usize, color.to_scalar(), thickness)
        }
    }

    pub fn draw_contour<T: ToScalar>(&mut self, contour: &Contour, color: T, thickness: i32) {
        let mut contours = Contours::new();
        contours.add(contour.clone());
        self.draw_contours(&contours, color, thickness);
    }

    pub fn draw_circle<C: ToScalar>(&mut self, center: &Point, radius: i32, color: C, thickness: i32) {
        circle(
            &mut self.inner,
            center.to_rustcv(),
            radius,
            color.to_scalar(),
            thickness,
        );
    }

    pub fn draw_rect<C: ToScalar>(&mut self, rect: &Rect, color: C, thickness: i32) {
        rustcv::imgproc::rectangle(&mut self.inner, rect.to_rustcv(), color.to_scalar(), thickness)
    }

    pub fn draw_line<C: ToScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32) {
        line(
            &mut self.inner,
            p1.to_rustcv(),
            p2.to_rustcv(),
            color.to_scalar(),
            thickness,
        );
    }

    pub fn median_blur(&mut self, size: i32) {
        let size = if size % 2 == 0 {
            let new_size = size + 1;
            warn!(
                "median_blur called with an even value: {} - expected an odd value - i use {} instead!",
                size, new_size
            );
            new_size
        } else {
            size
        };

        rustcv::imgproc::median_blur(&self.inner.clone(), &mut self.inner, size);
    }

    pub fn apply_mask(&self, mask: &Mat<Gray>) -> Mat<ColorSpace> {
        let mut res = RustCVMat::new();
        self.inner.copy_to_with_mask(&mut res, &mask.inner);
        Mat::from_rustcv(res)
    }
}

pub trait ToHSV {
    fn to_hsv(&self) -> Mat<HSV>;
    fn convert<ColorSpace>(mat: &Mat<ColorSpace>, conversion: ColorConversion) -> Mat<HSV> {
        let mut hsv = RustCVMat::new();
        cvt_color(&mat.inner, &mut hsv, conversion);
        Mat::from_rustcv(hsv)
    }
}

impl ToHSV for Mat<BGR> where {
    fn to_hsv(&self) -> Mat<HSV> {
        Self::convert(self, ColorConversion::BGR2HSV)
    }
}

impl ToHSV for Mat<RGB> where {
    fn to_hsv(&self) -> Mat<HSV> {
        Self::convert(self, ColorConversion::RGB2HSV)
    }
}

pub trait FindCenter {
    fn find_center(&self) -> Option<Point>;
}

impl FindCenter for Mat<Gray> where {
    fn find_center(&self) -> Option<Point> {
        let moments = moments(&self.inner, true);
        if moments.m00 > 0.0 {
            let center = Point {
                x: (moments.m10 / moments.m00) as i32,
                y: (moments.m01 / moments.m00) as i32,
            };
            Some(center)
        } else {
            None
        }
    }
}

pub trait InRange {
    fn in_range<T: ToScalar>(&self, lb: &T, ub: &T) -> Mat<Gray>;
}

impl InRange for Mat<HSV> where {
    fn in_range<T: ToScalar>(&self, lb: &T, ub: &T) -> Mat<Gray> {
        let mut masked = RustCVMat::new();
        in_range_with_scalar(&self.inner, lb.to_scalar(), ub.to_scalar(), &mut masked);
        Mat::from_rustcv(masked)
    }
}

pub trait FindContours {
    fn find_contours(&self) -> Contours;
}
impl FindContours for Mat<Gray> where {
    fn find_contours(&self) -> Contours {
        let contours =
            rustcv::imgproc::find_contours(&self.inner, RetrievalMode::CComp, ContourApproximationMode::Simple);

        Contours::from_rustcv(contours)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::path::Path;

    #[test]
    fn point_to_from_rustcv() {
        let p = Point::new(300, 400);
        assert_eq!(p, Point::from_rustcv(p.to_rustcv()));
    }

    #[test]
    fn contour_to_from_rustcv() {
        let c = Contour((0..10).map(|i| Point::new(i, i * 10)).collect());
        assert_eq!(c, Contour::from_rustcv(c.to_rustcv()));
    }

    #[test]
    fn contours_to_from_rustcv() {
        let mut cs = Contours::new();
        cs.add(Contour((0..10).map(|i| Point::new(i, i * 10)).collect()));
        cs.add(Contour((10..20).map(|i| Point::new(i, i * 10)).collect()));

        assert_eq!(cs, Contours::from_rustcv(cs.to_rustcv()));
    }

    #[test]
    fn bonding_rect() {
        let c = Contour(vec![Point::new(100, 200), Point::new(500, 800)]);
        let rect = c.bounding_rect();
        assert_eq!(
            rect,
            Rect {
                x: 100,
                y: 200,
                width: 401,  // FIXME 401?
                height: 601, // FIXME 601?
            }
        );
    }

}
