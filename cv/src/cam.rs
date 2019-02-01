use crate::*;
use rustcv::videoio::VideoCapture;

/// Video capturing from cameras.
#[derive(Debug)]
pub struct Cam {
    cam: VideoCapture,
}

impl Cam {
    /// Opens the first connected cam.
    pub fn new() -> Result<Cam, CVErr> {
        Self::new_for_device_id(0)
    }

    /// Opens the cam with the given 'device_id'
    pub fn new_for_device_id(device_id: i32) -> Result<Cam, CVErr> {
        let cam = VideoCapture::new();
        if cam.open_device(device_id) {
            Ok(Cam { cam })
        } else {
            Err(CVErr::cam_err(format!(
                "unable to open cam with device_id: {}",
                device_id
            )))
        }
    }

    /// Try to grab a image from the cam
    pub fn grab(&mut self) -> Option<Mat<BGR>> {
        self.cam.grab().map(Mat::from_rustcv)
    }
}
