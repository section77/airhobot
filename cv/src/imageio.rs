use crate::*;
use std::path::PathBuf;

pub fn imread<P>(p: P) -> Result<Mat<BGR>>
where
    P: Into<PathBuf>,
{
    let path = p.into();

    if !path.exists() {
        return Err(Error::VideoIO {
            source: opencv::Error::new(0, format!("unable to open file: {}", path.to_string_lossy())),
        });
    }

    // https://docs.opencv.org/master/d6/d87/imgcodecs_8hpp.html
    let imread_unchanged = -1;

    Ok(Mat::pack(opencv::imgcodecs::imread(
        &path.to_string_lossy(),
        imread_unchanged,
    )?))
}
