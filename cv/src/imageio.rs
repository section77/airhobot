use crate::*;
use std::path::Path;

pub fn imread<Type>(p: &Path) -> Result<Mat<Type>> {
    // https://docs.opencv.org/master/d6/d87/imgcodecs_8hpp.html
    let imread_unchanged = -1;

    Ok(Mat::wrap(opencv::imgcodecs::imread(
        &p.to_string_lossy(),
        imread_unchanged,
    )?))
}
