use crate::*;
use opencv::videoio;

/// Video capturing from cameras.
#[derive(Debug)]
pub struct Cam {
    cam: videoio::VideoCapture,
}

impl Cam {
    /// Opens the first connected cam.
    pub fn new() -> Result<Cam, CVErr> {
        Self::new_for_device_id(0)
    }

    /// Opens the cam with the given 'device_id'
    pub fn new_for_device_id(device_id: i32) -> Result<Cam, CVErr> {
        let cam = videoio::VideoCapture::new_with_backend(device_id, videoio::CAP_ANY);
        if videoio::VideoCapture::is_opened(&cam)? {
            Ok(Cam { cam })
        } else {
            Err(CVErr::cam_err(format!(
                "unable to open cam with device_id: {}",
                device_id
            )))
        }
    }

    /// Try to grab a image from the cam
    pub fn grab(&mut self) -> Result<Mat<BGR>> {
        let mut frame = opencv::core::Map::default()?;
        self.cam.read(&mut frame)?;
        Map::wrap(frame)
        self.cam.grab().map(Mat::from_rustcv)
    }
}
