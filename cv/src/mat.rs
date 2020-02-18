use super::*;
use opencv::core::Mat as OpencvMat;
use std::marker::PhantomData;
use log::debug;

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
        let n_rows = inner.rows().unwrap();
        let n_cols = inner.cols().unwrap();
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

    pub fn roi(m: &Mat<ColorSpace>, roi: Rect) -> Result<Mat<ColorSpace>> {
	Ok(Mat::pack(opencv::core::Mat::roi(&m.inner, roi.unpack())?))
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

    // FIXME: ColorSpace
    pub fn at_2d(&self, x: i32, y: i32) -> Result<HSV> {
        let v = self.inner.at_2d::<opencv::core::Vec3b>(y, x)?;
        debug!("requested point at: x: {}, y: {}, value: {:?}", x, y, v);
        //	Ok(HSV::unsafe_new(v[0] as u8, v[1] as u8, v[2] as u8))
        let rgb = (RGB::new(v[2] as u8, v[1] as u8, v[0] as u8));
        let hsv = (Mat::<HSV>::rgb2hsv(rgb));
        Ok(hsv)
    }

    fn rgb2hsv(rgb: RGB) -> HSV {
        let (r, g, b) = (rgb.r as f32 / 255.0,
                         rgb.g as f32 / 255.0,
                         rgb.b as f32 / 255.0);
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let diff = max - min;

        let mut h = 60.0 *
            if max == min {
                0.0
            } else if max == r {
                (g-b) / diff
            } else if max == g {
                2.0 + (b - r) / diff
            } else if max == b {
                4.0 + (r - g) / diff
            } else {
                unreachable!()
            };
        if h < 0.0 {
            h += 360.0;
        }

        let s = if max == 0.0 {
            0.0
        } else {
            diff / max
        };

        let h = ((h / 2.0).max(0.0));
        let s = (s);
        let v = (max);


        HSV::unsafe_new(h as u8, (s * 255.0) as u8, (v * 255.0) as u8)
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

    pub fn draw_text<C>(&mut self, text: &str, org: &Point, font_scale: f64, color: C, thickness: i32) -> Result<()>
    where C: ToOpencvScalar,
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
            bottom_left_origin
        )?;
        Ok(())
    }

    pub fn draw_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32) -> Result<()>{
        let line_filled = -1;
        let shift = 0;
        opencv::imgproc::line(
            &mut self.inner,
            p1.unpack(),
            p2.unpack(),
            color.to_opencv_scalar(),
            thickness,
            line_filled,
            shift
        )?;
        Ok(())
    }


    pub fn draw_arrowed_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: &C, thickness: i32) -> Result<()>{
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
            tip_length
        )?;
        Ok(())
    }

    pub fn copy_to(&self, target: &mut Mat<ColorSpace>) -> Result<()> {
	self.unpack().copy_to(&mut target.inner)?;
	Ok(())
    }

    pub fn blur(&mut self, ksize: i32) -> Result<()> {
        let src = self.inner.clone().unwrap();
        opencv::imgproc::blur(&src, &mut self.inner, opencv::core::Size::new(ksize, ksize)
                              , opencv::core::Point::new(-1, -1), 4)?;

        Ok(())
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
    fn in_range<T: ToOpencvScalar>(&self, lb: &T, ub: &T) -> Mat<Gray>;
}

impl InRange for Mat<HSV> {
    fn in_range<T: ToOpencvScalar>(&self, lb: &T, ub: &T) -> Mat<Gray> {
        let mut masked = OpencvMat::default().unwrap();
        opencv::core::in_range(&self.inner, &lb.to_opencv_scalar(), &ub.to_opencv_scalar(), &mut masked);
        Mat::pack(masked)
    }
}

pub trait FindContours {
    fn find_contours(&mut self) -> Result<Contours>;
}
impl FindContours for Mat<Gray> {
    fn find_contours(&mut self) -> Result<Contours> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb2hsv() {
        let res = dbg!(Mat::<HSV>::rgb2hsv(RGB::new(11, 75, 143)));
        assert_eq!(res, HSV::unsafe_new(210, 92, 56));
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
