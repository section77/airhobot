use crate::{prelude::*, Mat, Point, Rect, ToOpencvScalar, Contours};
use opencv::prelude::Vector;

pub trait Draw {
    fn draw_contours<C: ToOpencvScalar>(&mut self, contours: &Contours, color: C, thickness: i32);
    fn draw_circle<C: ToOpencvScalar>(&mut self, center: &Point, radius: i32, color: C, thickness: i32);
    fn draw_rect<C: ToOpencvScalar>(&mut self, rect: &Rect, color: C, thickness: i32);
    fn draw_text<C: ToOpencvScalar>(&mut self, text: &str, org: &Point, font_scale: f64, color: C, thickness: i32);
    fn draw_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32);
    fn draw_arrowed_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: &C, thickness: i32);
}

impl<T> Draw for Mat<T> {
    fn draw_contours<C: ToOpencvScalar>(&mut self, contours: &Contours, color: C, thickness: i32) {
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

    fn draw_circle<C: ToOpencvScalar>(&mut self, center: &Point, radius: i32, color: C, thickness: i32) {
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

    fn draw_rect<C: ToOpencvScalar>(&mut self, rect: &Rect, color: C, thickness: i32) {
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

    fn draw_text<C>(&mut self, text: &str, org: &Point, font_scale: f64, color: C, thickness: i32)
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

    fn draw_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: C, thickness: i32) {
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

    fn draw_arrowed_line<C: ToOpencvScalar>(&mut self, p1: &Point, p2: &Point, color: &C, thickness: i32) {
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

}
