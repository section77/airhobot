use crate::*;
use opencv::{
    core::Mat as OpencvMat,
    imgproc::ColorConversionCodes::{self, *},
};


pub trait ConvertColor<To> {
    fn convert_color(&self) -> Mat<To>;
}

impl ConvertColor<HSV> for Mat<BGR> {
    fn convert_color(&self) -> Mat<HSV> {
        convert(self, COLOR_BGR2HSV)
    }
}

impl ConvertColor<BGR> for Mat<HSV> {
    fn convert_color(&self) -> Mat<BGR> {
        convert(self, COLOR_HSV2BGR)
    }
}

impl ConvertColor<BGR> for Mat<Gray> {
    fn convert_color(&self) -> Mat<BGR> {
        convert(self, COLOR_GRAY2BGR)
    }
}

fn convert<From, To>(mat: &Mat<From>, code: ColorConversionCodes) -> Mat<To> {
    let mut to = OpencvMat::default().expect("Mat::default()");
    opencv::imgproc::cvt_color(&mat.inner, &mut to, code as i32, 0).expect("cvt_color");
    Mat::pack(to)
}
