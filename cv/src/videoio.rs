use crate::*;
use log::warn;
use opencv::videoio::VideoCapture as OpencvVideoCapture;
use opencv::videoio::VideoWriter as OpencvVideoWriter;
use std::path::Path;

pub struct VideoCapture {
    inner: OpencvVideoCapture,
}

impl VideoCapture {
    pub fn open_file(path: &Path) -> Result<VideoCapture> {
        let mut inner = OpencvVideoCapture::default()?;
        if inner.open_file_with_backend(&path.to_string_lossy(), opencv::videoio::CAP_ANY)? {
            Ok(VideoCapture { inner })
        } else {
            // FIXME
            Err(CVErr::new(
                Component::VideoCapture,
                format!("unable to open file: {}", path.to_string_lossy()),
            ))
        }
    }

    pub fn open_device(device_id: i32) -> Result<VideoCapture> {
        let mut inner = OpencvVideoCapture::default()?;
        if inner.open_with_backend(device_id, opencv::videoio::CAP_ANY)? {
            Ok(VideoCapture { inner })
        } else {
            Err(CVErr::new(
                Component::VideoCapture,
                format!("unable to open device with device_id: {}", device_id),
            ))
        }
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
            return None
        }
        return Some(mat)
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
            Err(CVErr::new(
                Component::VideoWriter,
                "unable to open video-writer".to_string(),
            ))
        }
    }

    pub fn write(&mut self, frame: &Mat<BGR>) {
        let check = |name, actual, expected| {
            if actual != expected {
                warn!(
                    "wrong {} - frame-height: {}, VideoWriter expects: {} - this will result in an empty video",
                    name, actual, expected,
                )
            }
        };

        check("width", frame.n_cols(), self.width);
        check("height", frame.n_rows(), self.height);

        self.inner.write(frame.unpack());
    }
}
