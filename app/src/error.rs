use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {

    #[snafu(display("Invalid argument: {}", msg))]
    Arguments { msg: String },

    #[snafu(display("Error: {}", msg))]
    Generic { msg: String },

    #[snafu(display("{}", source))]
    Cv { source: cv::Error },

    #[snafu(display("IO error: {}", source))]
    IOErr { source: std::io::Error },

    #[snafu(display("Serde error: {}", source))]
    Serde { source: serde_json::Error },
}

impl From<cv::Error> for Error {
    fn from(source: cv::Error) -> Self {
        Error::Cv { source }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Generic { msg }
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Generic { msg: msg.to_string() }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error::IOErr { source }
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::Serde { source }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(source: std::sync::PoisonError<T>) -> Self {
        let msg = format!("lock poison - {}", source);
        Error::Generic { msg }
    }
}

impl From<crossbeam_channel::RecvError> for Error {
    fn from(source: crossbeam_channel::RecvError) -> Self {
        let msg = format!("crossbeam recv error - {}", source);
        Error::Generic { msg }
    }
}
