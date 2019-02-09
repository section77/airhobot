use crate::Mat;
use std::path::Path;

//FIXME: error type
pub fn imread<Type>(p: &Path) -> Result<Mat<Type>, ()> {
    // FIXME: this returns no error, when the image was not found!
    rustcv::imgcodecs::imread(p, rustcv::imgcodecs::ImageReadMode::Unchanged)
        .map(Mat::from_rustcv)
        .map_err(|_e| ())
}
