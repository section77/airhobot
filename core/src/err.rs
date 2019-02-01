use std::{convert, error, fmt};
pub use AirHoBotErr::*;

#[derive(Debug)]
pub enum AirHoBotErr {
    AppErr(String),
    OpenCVErr(String),
    IOErr(String),
}

impl fmt::Display for AirHoBotErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppErr(msg) => write!(f, "{}", msg),
            OpenCVErr(msg) => write!(f, "opencv-error: {}", msg),
            IOErr(msg) => write!(f, "io-error: {}", msg),
        }
    }
}

impl error::Error for AirHoBotErr {
    fn description(&self) -> &str {
        match self {
            AppErr(msg) => msg.as_str(),
            OpenCVErr(msg) => msg.as_str(),
            IOErr(msg) => msg.as_str(),
        }
    }
}

impl convert::From<cv::CVErr> for AirHoBotErr {
    fn from(cv_err: cv::CVErr) -> Self {
        OpenCVErr(format!("{:?}: {}", cv_err.component, cv_err.msg))
    }
}

impl convert::From<std::io::Error> for AirHoBotErr {
    fn from(err: std::io::Error) -> Self {
        IOErr(format!("{:?}", err))
    }
}
