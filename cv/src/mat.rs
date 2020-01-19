use log::warn;
use opencv::core::Mat as OpencvMat;
// use rustcv::core::in_range_with_scalar;
// use rustcv::core::Mat as RustCVMat;
// use rustcv::imgproc::*;

use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Gray;

#[derive(Debug)]
pub struct Mat<ColorSpace> {
    inner: OpencvMat,
    n_rows: i32,
    n_cols: i32,
    _type: PhantomData<ColorSpace>,
}

impl<ColorSpace> Mat<ColorSpace> {
    pub fn new<C: ToOpencvScalar>(rows: i32, cols: i32, cv_type: &CVType, background: C) -> Result<Mat<ColorSpace>> {
        Ok(Mat::wrap(OpencvMat::new_rows_cols_with_default(
            rows,
            cols,
            cv_type.unpack(),
            background.to_opencv_scalar(),
        )?))
    }

    pub fn wrap(inner: opencv::core::Mat) -> Mat<ColorSpace> {
        let n_rows = inner.rows().unwrap();
        let n_cols = inner.cols().unwrap();
        Mat {
            inner,
            n_rows,
            n_cols,
            _type: PhantomData,
        }
    }

    pub fn unwrap(&self) -> &opencv::core::Mat {
        &self.inner
    }

    pub fn n_rows(&self) -> i32 {
        self.n_rows
    }

    pub fn n_cols(&self) -> i32 {
        self.n_cols
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.inner.empty()?)
    }

    // pub fn draw_contours<T: ToOpencvScalar>(&mut self, contours: &Contours, color: T, thickness: i32) {
    //     let contours = contours.unwrap();
    //     for idx in 0..contours.length {
    //         rustcv::imgproc::draw_contours(&mut self.inner, contours, idx as usize, color.to_scalar(), thickness)
    //     }
    // }

    // pub fn draw_contour<T: ToOpencvScalar>(&mut self, contour: &Contour, color: T, thickness: i32) {
    //     let mut contours = Contours::new();
    //     contours.add(contour.clone());
    //     self.draw_contours(&contours, color, thickness);
    // }

    pub fn draw_circle<C: ToOpencvScalar>(&mut self, center: &Point, radius: i32, color: C, thickness: i32) {
        let line_filled = -1;
        let shift = 0;
        opencv::imgproc::circle(
            &mut self.inner,
            center.unpack(),
            radius,
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            shift,
        )
        .unwrap();
    }

    pub fn draw_rect<C: ToOpencvScalar>(&mut self, rect: &Rect, color: C, thickness: i32) -> Result<()> {
        let line_filled = -1;
        let shift = 0;
        opencv::imgproc::rectangle(
            &mut self.inner,
            rect.unpack(),
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            shift,
        )?;
        Ok(())
    }

    // pub fn draw_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32) {
    //     line(
    //         &mut self.inner,
    //         p1.to_rustcv(),
    //         p2.to_rustcv(),
    //         color.to_scalar(),
    //         thickness,
    //     );
    // }

    // pub fn median_blur(&mut self, size: i32) {
    //     let size = if size % 2 == 0 {
    //         let new_size = size + 1;
    //         warn!(
    //             "median_blur called with an even value: {} - expected an odd value - i use {} instead!",
    //             size, new_size
    //         );
    //         new_size
    //     } else {
    //         size
    //     };

    //     rustcv::imgproc::median_blur(&self.inner.clone(), &mut self.inner, size);
    // }

    // pub fn apply_mask(&self, mask: &Mat<Gray>) -> Mat<ColorSpace> {
    //     let mut res = RustCVMat::new();
    //     self.inner.copy_to_with_mask(&mut res, &mask.inner);
    //     Mat::from_rustcv(res)
    // }
}

pub trait ToHSV {
    fn to_hsv(&self) -> Result<Mat<HSV>>;
    // codes, see: https://github.com/twistedfall/opencv-rust/blob/98405e0502af98eda0879ac799c6bb9abc5ca77d/headers/3.4/opencv2/imgproc.hpp#L525
    fn convert<ColorSpace>(mat: &Mat<ColorSpace>, code: i32) -> Result<Mat<HSV>> {
        let mut hsv = OpencvMat::default()?;
        opencv::imgproc::cvt_color(&mat.inner, &mut hsv, code, 0)?;
        Ok(Mat::wrap(hsv))
    }
}

impl ToHSV for Mat<BGR> {
    fn to_hsv(&self) -> Result<Mat<HSV>> {
        Self::convert(self, 40)
    }
}

impl ToHSV for Mat<RGB> {
    fn to_hsv(&self) -> Result<Mat<HSV>> {
        Self::convert(self, 41)
    }
}

// pub trait FindCenter {
//     fn find_center(&self) -> Option<Point>;
// }

// impl FindCenter for Mat<Gray> where {
//     fn find_center(&self) -> Option<Point> {
//         let moments = moments(&self.inner, true);
//         if moments.m00 > 0.0 {
//             let center = Point {
//                 x: (moments.m10 / moments.m00) as i32,
//                 y: (moments.m01 / moments.m00) as i32,
//             };
//             Some(center)
//         } else {
//             None
//         }
//     }
// }

pub trait InRange {
    fn in_range<T: ToOpencvScalar>(&self, lb: &T, ub: &T) -> Mat<Gray>;
}

impl InRange for Mat<HSV> {
    fn in_range<T: ToOpencvScalar>(&self, lb: &T, ub: &T) -> Mat<Gray> {
        let mut masked = OpencvMat::default().unwrap();
        opencv::core::in_range(&self.inner, &lb.to_opencv_scalar(), &ub.to_opencv_scalar(), &mut masked);
        Mat::wrap(masked)
    }
}

pub trait FindContours {
    fn find_contours(&mut self) -> Result<Contours>;
}
impl FindContours for Mat<Gray> {
    fn find_contours(&mut self) -> Result<Contours> {
        use opencv::prelude::Vector;
        let mut contours = opencv::types::VectorOfVectorOfPoint::new();
        opencv::imgproc::find_contours(
            &mut self.inner,
            &mut contours,
            opencv::imgproc::RETR_CCOMP,
            opencv::imgproc::CHAIN_APPROX_SIMPLE,
            OpencvPoint::new(0, 0),
        )?;
        Ok(Contours::pack(contours))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::*;
//     use std::path::Path;

//     #[test]
//     fn point_to_from_rustcv() {
//         let p = Point::new(300, 400);
//         assert_eq!(p, Point::from_rustcv(p.to_rustcv()));
//     }

//     #[test]
//     fn contour_to_from_rustcv() {
//         let c = Contour((0..10).map(|i| Point::new(i, i * 10)).collect());
//         assert_eq!(c, Contour::from_rustcv(c.to_rustcv()));
//     }

//     #[test]
//     fn contours_to_from_rustcv() {
//         let mut cs = Contours::new();
//         cs.add(Contour((0..10).map(|i| Point::new(i, i * 10)).collect()));
//         cs.add(Contour((10..20).map(|i| Point::new(i, i * 10)).collect()));

//         assert_eq!(cs, Contours::from_rustcv(cs.to_rustcv()));
//     }

//     #[test]
//     fn bonding_rect() {
//         let c = Contour(vec![Point::new(100, 200), Point::new(500, 800)]);
//         let rect = c.bounding_rect();
//         assert_eq!(
//             rect,
//             Rect {
//                 x: 100,
//                 y: 200,
//                 width: 401,  // FIXME 401?
//                 height: 601, // FIXME 601?
//             }
//         );
//     }

// }
