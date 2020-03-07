use super::*;
use log::debug;
use opencv::{core::Mat as OpencvMat, core::Point as OpencvPoint, prelude::MatTrait};
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
        Ok(Mat::pack(OpencvMat::new_rows_cols_with_default(
            rows,
            cols,
            cv_type.unpack(),
            background.to_opencv_scalar(),
        )?))
    }

    pub fn pack(inner: opencv::core::Mat) -> Mat<ColorSpace> {
        let n_rows = inner.rows();
        let n_cols = inner.cols();
        Mat {
            inner,
            n_rows,
            n_cols,
            _type: PhantomData,
        }
    }

    pub fn unpack(&self) -> &opencv::core::Mat {
        &self.inner
    }

    pub fn roi(&self, roi: Rect) -> Result<Mat<ColorSpace>> {
        Ok(Mat::pack(opencv::core::Mat::roi(&self.inner, roi.unpack())?))
    }

    pub fn lens(&self, points: &[Point; 4]) -> Result<Mat<ColorSpace>> {
        use opencv::{calib3d::find_homography, imgproc::warp_perspective, types::VectorOfPoint};

        let w = points[1].x - points[0].x;
        let h = points[2].y - points[1].y;
        let mut dst_corners = VectorOfPoint::with_capacity(4);
        dst_corners.push(OpencvPoint::new(0, 0));
        dst_corners.push(OpencvPoint::new(w, 0));
        dst_corners.push(OpencvPoint::new(w, h));
        dst_corners.push(OpencvPoint::new(0, h));

        let roi_corners_mat = OpencvMat::from_exact_iter(points.iter().map(|p| p.unpack()))?;
        let dst_corners_mat = OpencvMat::from_exact_iter(dst_corners.iter())?;

        let hom = find_homography(&roi_corners_mat, &dst_corners_mat, &mut OpencvMat::default()?, 0, 3.)?;
        let mut warped = OpencvMat::default()?;
        let size = opencv::core::Size::new(w, h);
        // https://docs.rs/opencv/0.30.1/opencv/imgproc/fn.warp_perspective.html
        warp_perspective(
            &self.inner,
            &mut warped,
            &hom,
            size,
            opencv::imgproc::INTER_LINEAR,
            opencv::core::BORDER_CONSTANT,
            opencv::core::Scalar::default(),
        )?;

        Ok(Mat::pack(warped))
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

    pub fn at(&self, p: &Point) -> Result<HSV> {
        let vec3b = self.inner.at_2d::<opencv::core::Vec3b>(p.y, p.x)?;
        let hsv = HSV::from(RGB::from(*vec3b));
        debug!("color at point: {} is: {}", p, hsv);
        Ok(hsv)
    }

    pub fn at_avg(&self, p: &Point, size: u8) -> Result<HSV> {
        let size = size as i32;
        let offset = size / 2;
        let (x, y) = if p.x - offset > 0 && p.y - offset > 0 {
            (p.x - offset, p.y - offset)
        } else {
            (p.x, p.y)
        };

        let (mut h, mut s, mut v) = (0, 0, 0);
        for x in x..(x+size) {
            for y in y..(y+size) {
                let hsv = self.at(&Point::new(x, y))?;
                h += hsv.h as i32;
                s += hsv.s as i32;
                v += hsv.v as i32;
            }
        }
        let n = size * size;
        HSV::new((h / n) as u8, (s / n) as u8, (v / n) as u8)
    }

    pub fn draw_contours<T: ToOpencvScalar>(&mut self, contours: &Contours, color: T, thickness: i32) {
        let contours = contours.unpack();
        let line_filled = -1;
        let hierarchy = opencv::types::VectorOfPoint::new();
        let max_level = 0;
        let shift = opencv::core::Point::default();
        for idx in 0..contours.len() as i32 {
            opencv::imgproc::draw_contours(
                &mut self.inner,
                contours,
                idx,
                color.to_opencv_scalar(),
                thickness,
                line_filled,
                &hierarchy,
                max_level,
                shift,
            )
            .expect("unable to draw_contours");
        }
    }

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
        .expect("unable to draw_circle");
    }

    pub fn draw_rect<C: ToOpencvScalar>(&mut self, rect: &Rect, color: C, thickness: i32) {
        let line_filled = -1;
        let shift = 0;
        opencv::imgproc::rectangle(
            &mut self.inner,
            rect.unpack(),
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            shift,
        )
        .expect("unable to draw_rect");
    }

    pub fn draw_text<C>(&mut self, text: &str, org: &Point, font_scale: f64, color: C, thickness: i32)
    where
        C: ToOpencvScalar,
    {
        let font_hershey_simplex = 0;
        let line_filled = -1;
        let bottom_left_origin = false;
        opencv::imgproc::put_text(
            &mut self.inner,
            text,
            org.unpack(),
            font_hershey_simplex,
            font_scale,
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            bottom_left_origin,
        )
        .expect("unable to draw_text");
    }

    pub fn draw_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32) {
        let line_filled = -1;
        let shift = 0;
        opencv::imgproc::line(
            &mut self.inner,
            p1.unpack(),
            p2.unpack(),
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            shift,
        )
        .expect("unable to draw_line");
    }

    pub fn draw_arrowed_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: &C, thickness: i32) {
        let line_filled = -1;
        let shift = 0;
        let tip_length = 1.0;
        opencv::imgproc::arrowed_line(
            &mut self.inner,
            p1.unpack(),
            p2.unpack(),
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            shift,
            tip_length,
        )
        .expect("unable to draw_arrowed_line");
    }

    pub fn copy_to(&self, target: &mut Mat<ColorSpace>) -> Result<()> {
        self.unpack().copy_to(&mut target.inner)?;
        Ok(())
    }

    pub fn blur(&mut self, ksize: i32) {
        if ksize == 0 {
            return;
        }

        let src = self.inner.clone().unwrap();
        let kernel = opencv::core::Size::new(ksize, ksize);
        let anchor = opencv::core::Point::new(-1, -1);
        opencv::imgproc::blur(&src, &mut self.inner, kernel, anchor, 4).expect("blur");
    }

    pub fn erode(&mut self, ksize: i32) {
        if ksize == 0 {
            return;
        }

        let src = self.inner.clone().unwrap();

        let kernel = opencv::imgproc::get_structuring_element(
            opencv::imgproc::MORPH_ELLIPSE,
            opencv::core::Size::new(ksize, ksize),
            opencv::core::Point::new(-1, -1),
        )
        .unwrap();
        let anchor = opencv::core::Point::new(-1, -1);
        let iterations = 1;
        // https://docs.opencv.org/4.2.0/d2/de8/group__core__array.html#ga209f2f4869e304c82d07739337eae7c5
        let border_type = opencv::core::BORDER_CONSTANT;
        let border_value = opencv::imgproc::morphology_default_border_value().expect("morphology_default_border_value");
        opencv::imgproc::erode(
            &src,
            &mut self.inner,
            &kernel,
            anchor,
            iterations,
            border_type,
            border_value,
        )
        .expect("erode");
    }

    pub fn dilate(&mut self, ksize: i32) {
        if ksize == 0 {
            return;
        }

        let src = self.inner.clone().unwrap();

        let kernel = opencv::imgproc::get_structuring_element(
            opencv::imgproc::MORPH_ELLIPSE,
            opencv::core::Size::new(ksize, ksize),
            opencv::core::Point::new(-1, -1),
        )
        .unwrap();
        let anchor = opencv::core::Point::new(-1, -1);
        let iterations = 1;
        // https://docs.opencv.org/4.2.0/d2/de8/group__core__array.html#ga209f2f4869e304c82d07739337eae7c5
        let border_type = opencv::core::BORDER_CONSTANT;
        let border_value = opencv::imgproc::morphology_default_border_value().expect("morphology_default_border_value");
        opencv::imgproc::dilate(
            &src,
            &mut self.inner,
            &kernel,
            anchor,
            iterations,
            border_type,
            border_value,
        )
        .expect("dilate");
    }

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

impl<ColorSpace> Clone for Mat<ColorSpace> {
    fn clone(&self) -> Self {
        Self {
            inner: OpencvMat::clone(&self.inner).unwrap(),
            n_rows: self.n_rows,
            n_cols: self.n_cols,
            _type: self._type.clone(),
        }
    }
}

pub trait ToHSV {
    fn to_hsv(&self) -> Result<Mat<HSV>>;
    // codes, see: https://github.com/twistedfall/opencv-rust/blob/98405e0502af98eda0879ac799c6bb9abc5ca77d/headers/3.4/opencv2/imgproc.hpp#L525
    fn convert<ColorSpace>(mat: &Mat<ColorSpace>, code: i32) -> Result<Mat<HSV>> {
        let mut hsv = OpencvMat::default()?;
        opencv::imgproc::cvt_color(&mat.inner, &mut hsv, code, 0)?;
        Ok(Mat::pack(hsv))
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
    fn in_range(&self, range: &HSVRange) -> Mat<Gray>;
}

impl InRange for Mat<HSV> {
    fn in_range(&self, range: &HSVRange) -> Mat<Gray> {
        let mut masked = OpencvMat::default().unwrap();
        opencv::core::in_range(
            &self.inner,
            &range.min().to_opencv_scalar(),
            &range.max().to_opencv_scalar(),
            &mut masked,
        )
        .expect("in_range error");
        Mat::pack(masked)
    }
}

pub trait FindContours {
    fn find_contours(&mut self) -> Contours;
}
impl FindContours for Mat<Gray> {
    fn find_contours(&mut self) -> Contours {
        let mut contours = opencv::types::VectorOfVectorOfPoint::new();
        opencv::imgproc::find_contours(
            &mut self.inner,
            &mut contours,
            opencv::imgproc::RETR_CCOMP,
            opencv::imgproc::CHAIN_APPROX_SIMPLE,
            OpencvPoint::new(0, 0),
        )
        .expect("unable to find_countours");
        Contours::pack(contours)
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
