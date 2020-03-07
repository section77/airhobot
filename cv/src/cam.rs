use crate::*;
use opencv::{videoio, prelude::VideoCaptureTrait};

/// Video capturing from cameras.
pub struct Cam {
    cam: videoio::VideoCapture,
}

impl Cam {
    /// Opens the first connected cam.
    pub fn new() -> Result<Cam> {
        Self::new_for_device_id(0)
    }

    /// Opens the cam with the given 'device_id'
    pub fn new_for_device_id(device_id: i32) -> Result<Cam> {
        let cam = videoio::VideoCapture::new(device_id, videoio::CAP_ANY)?;
        if videoio::VideoCapture::is_opened(&cam)? {
            Ok(Cam { cam })
        } else {
            Err(Error::Cam {
                source: opencv::Error::new(0, format!("unable to open cam with device_id: {}", device_id)),
            })
        }
    }

    /// Try to grab a image from the cam
    pub fn grab(&mut self) -> Result<Mat<BGR>> {
        let mut frame = opencv::core::Mat::default()?;
        self.cam.read(&mut frame)?;
        Ok(Mat::pack(frame))
    }
}

impl Iterator for Cam {
    type Item = Mat<BGR>;
    fn next(&mut self) -> Option<Self::Item> {
        let mat = self.grab().ok()?;
        if mat.is_empty().ok()? {
            return None;
        }
        return Some(mat);
    }
}
