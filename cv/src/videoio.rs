use crate::*;
use log::warn;
use std::path::Path;

/// FIXME
#[derive(Debug)]
pub struct VideoCapture {
    inner: rustcv::videoio::VideoCapture,
}

impl VideoCapture {
    pub fn open_file(path: &Path) -> Result<VideoCapture, CVErr> {
        let inner = rustcv::videoio::VideoCapture::new();
        if inner.open_file(path) {
            Ok(VideoCapture { inner })
        } else {
            // FIXME
            Err(CVErr::new(
                Component::VideoCapture,
                format!("unable to open file: {}", path.to_string_lossy()),
            ))
        }
    }

    pub fn open_device(device_id: i32) -> Result<VideoCapture, CVErr> {
        let inner = rustcv::videoio::VideoCapture::new();
        if inner.open_device(device_id) {
            Ok(VideoCapture { inner })
        } else {
            Err(CVErr::new(
                Component::VideoCapture,
                format!("unable to open device with device_id: {}", device_id),
            ))
        }
    }

    pub fn grab(&mut self) -> Option<Mat<BGR>> {
        self.inner.grab().map(Mat::from_rustcv)
    }
}

impl Iterator for VideoCapture {
    type Item = Mat<BGR>;
    fn next(&mut self) -> Option<Self::Item> {
        self.grab()
    }
}

/// FIXME
#[derive(Debug)]
pub struct VideoWriter {
    inner: rustcv::videoio::VideoWriter,
    width: i32,
    height: i32,
}

impl VideoWriter {
    pub fn new(path: &Path, fps: f64, width: i32, height: i32) -> Result<VideoWriter, CVErr> {
        let inner = rustcv::videoio::VideoWriter::new();
        inner.open(path, "XVID", fps, width, height, true);
        if inner.is_opened() {
            Ok(VideoWriter { inner, width, height })
        } else {
            Err(CVErr::new(
                Component::VideoWriter,
                "unable to open video-writer".to_string(),
            ))
        }
    }

    pub fn write(&self, frame: &Mat<BGR>) {
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

        self.inner.write(frame.to_rustcv());
    }
}
