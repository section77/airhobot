use crate::*;
use opencv::videoio::{
    VideoCapture as OpencvVideoCapture, VideoCaptureTrait, VideoWriter as OpencvVideoWriter, VideoWriterTrait, CAP_ANY,
};
use snafu::ensure;
use std::path::Path;

pub struct VideoCapture {
    inner: OpencvVideoCapture,
}

impl VideoCapture {
    pub fn open_file(path: &Path) -> Result<VideoCapture> {
        if !path.exists() {
            return Err(Error::VideoIO {
                source: opencv::Error::new(0, format!("unable to open file: {}", path.to_string_lossy())),
            });
        }

        let inner = OpencvVideoCapture::from_file(&path.to_string_lossy(), opencv::videoio::CAP_ANY)?;
        Ok(VideoCapture { inner })
    }

    pub fn open_device(device_id: i32) -> Result<VideoCapture> {
        let inner = OpencvVideoCapture::new(device_id, videoio::CAP_ANY)?;
        Ok(VideoCapture { inner })
    }

    pub fn grab(&mut self) -> Result<Mat<BGR>> {
        let mut frame = opencv::core::Mat::default()?;
        self.inner.read(&mut frame)?;
        Ok(Mat::pack(frame))
    }
}

impl Iterator for VideoCapture {
    type Item = Mat<BGR>;
    fn next(&mut self) -> Option<Self::Item> {
        let mat = self.grab().ok()?;
        if mat.is_empty().ok()? {
            return None;
        }
        return Some(mat);
    }
}

pub struct VideoWriter {
    inner: OpencvVideoWriter,
    width: i32,
    height: i32,
}

impl VideoWriter {
    pub fn new(path: &Path, fps: f64, width: i32, height: i32) -> Result<VideoWriter> {
        let mut inner = OpencvVideoWriter::default()?;

        let size = opencv::core::Size::new(width, height);
        let fourcc = OpencvVideoWriter::fourcc('X' as i8, 'V' as i8, 'I' as i8, 'D' as i8)?;
        if inner.open(&path.to_string_lossy(), fourcc, fps, size, true)? {
            Ok(VideoWriter { inner, width, height })
        } else {
            Err(Error::VideoIO {
                source: opencv::Error::new(0, format!("unable to video-writer")),
            })
        }
    }

    pub fn write(&mut self, frame: &Mat<BGR>) -> Result<()> {
        ensure!(
            frame.n_cols() == self.width,
            UserInput {
                msg: format!("width incompatible - frame: {}, video: {}", frame.n_cols(), self.width)
            }
        );

        ensure!(
            frame.n_rows() == self.height,
            UserInput {
                msg: format!(
                    "heigth incompatible - frame: {}, video: {}",
                    frame.n_rows(),
                    self.height
                )
            }
        );

        self.inner.write(frame.unpack())?;
        Ok(())
    }
}
