use super::Mat;
use rustcv::imgcodecs::*;
use std::error::Error;
use std::path::Path;

pub fn read<Type>(p: &Path) -> Result<Mat<Type>, impl Error> {
    imread(p, ImageReadMode::Unchanged)
        .map(Mat::from_rustcv)
        .map_err(|e| e.compat())
}
