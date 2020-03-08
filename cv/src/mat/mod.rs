use super::*;
use log::debug;
use opencv::{core::Mat as OpencvMat, core::Point as OpencvPoint, prelude::MatTrait};
use std::marker::PhantomData;

pub mod convert_color;
pub mod filter;
pub mod draw;

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
        for x in x..(x + size) {
            for y in y..(y + size) {
                let hsv = self.at(&Point::new(x, y))?;
                h += hsv.h as i32;
                s += hsv.s as i32;
                v += hsv.v as i32;
            }
        }
        let n = size * size;
        HSV::new((h / n) as u8, (s / n) as u8, (v / n) as u8)
    }

    pub fn copy_to(&self, target: &mut Mat<ColorSpace>) -> Result<()> {
        self.unpack().copy_to(&mut target.inner)?;
        Ok(())
    }
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
