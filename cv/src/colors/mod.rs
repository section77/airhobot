use opencv::core::Scalar as OpencvScalar;

mod bgr;
mod hsv;
mod hsv_range;
mod rgb;
pub use bgr::*;
pub use hsv::*;
pub use hsv_range::*;
pub use rgb::*;

pub trait ToOpencvScalar {
    fn to_opencv_scalar(&self) -> OpencvScalar;
}
