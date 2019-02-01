use rustcv::core::in_range_with_scalar;
use rustcv::core::Mat as RustCVMat;
use rustcv::imgproc::*;

use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct MASKED;

#[derive(Debug)]
pub struct Mat<Type> {
    inner: RustCVMat,
    n_rows: i32,
    n_cols: i32,
    _type: PhantomData<Type>,
}

impl<Type> Mat<Type> where {
    pub fn new() -> Mat<Type> {
        Self::from_rustcv(RustCVMat::new())
    }

    pub fn from_rustcv(inner: RustCVMat) -> Mat<Type> {
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

    pub fn draw_contours<T: ToScalar>(&mut self, mask: &Mat<MASKED>, color: T, thickness: i32) {
        let contours = find_contours(&mask.inner, RetrievalMode::CComp, ContourApproximationMode::Simple);
        for (idx, _) in contours.iter().enumerate() {
            draw_contours(&mut self.inner, &contours, idx, color.to_scalar(), thickness)
        }
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

    pub fn draw_line<C: ToScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32) {
        line(
            &mut self.inner,
            p1.to_rustcv(),
            p2.to_rustcv(),
            color.to_scalar(),
            thickness,
        );
    }

    pub fn apply_mask(&self, mask: &Mat<MASKED>) -> Mat<Type> {
        let mut res = RustCVMat::new();
        self.inner.copy_to_with_mask(&mut res, &mask.inner);
        Mat::from_rustcv(res)
    }
}

pub trait ToHSV {
    fn to_hsv(&self) -> Mat<HSV>;
    fn convert<T>(mat: &Mat<T>, conversion: ColorConversion) -> Mat<HSV> {
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

impl FindCenter for Mat<MASKED> where {
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
    fn in_range<T: ToScalar>(&self, lb: T, ub: T) -> Mat<MASKED>;
}

impl InRange for Mat<HSV> where {
    fn in_range<T: ToScalar>(&self, lb: T, ub: T) -> Mat<MASKED> {
        let mut masked = RustCVMat::new();
        in_range_with_scalar(&self.inner, lb.to_scalar(), ub.to_scalar(), &mut masked);
        Mat::from_rustcv(masked)
    }
}
