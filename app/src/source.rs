use crate::prelude::*;
use std::path::PathBuf;

/// Input source. Can be an Image, Video or Cam.
///
/// `Source` implements iterator, so you can easy loop
/// over the source and receive a new frame in each iteration.
///
/// ```
/// use airhobot::prelude::*;
/// let source = Source::cam(0);
/// for frame in source {
///   // process frame
/// }
/// ```
pub enum Source {
    Image(cv::Mat<cv::BGR>),
    Stream(Box<dyn Iterator<Item = cv::Mat<cv::BGR>>>),
}

impl Source {
    /// use the cam as input source.
    pub fn cam(device_id: i32) -> Result<Self> {
        let iter = cv::VideoCapture::open_device(device_id)?.into_iter();
        Ok(Self::Stream(Box::new(iter)))
    }

    /// use the video file as input source.
    pub fn video<P>(p: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let p = p.into();
        let iter = cv::VideoCapture::open_file(&p)?.into_iter();
        Ok(Self::Stream(Box::new(iter)))
    }

    /// use the image file as input source.
    ///
    /// the `Iterator` impl always returns the given image
    /// in each iteration.
    pub fn image<P>(p: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let p = p.into();
        let mat = cv::imread(p)?;
        Ok(Self::Image(mat))
    }

    pub fn grab(&mut self) -> Option<cv::Mat<cv::BGR>> {
        match self {
            Source::Image(frame) => Some(frame.clone()),
            Source::Stream(iter) => iter.next(),
        }
    }
}

impl Iterator for Source {
    type Item = cv::Mat<cv::BGR>;
    fn next(&mut self) -> Option<Self::Item> {
        self.grab()
    }
}
