use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("OpenCV error: {}", source))]
    OpenCV { source: opencv::Error },

    #[snafu(display("Cam error: {}", source))]
    Cam { source: opencv::Error },

    #[snafu(display("Videoio error: {}", source))]
    VideoIO { source: opencv::Error },

    #[snafu(display("User input error: {}", msg))]
    UserInput { msg: String },
}

impl From<opencv::Error> for Error {
    fn from(source: opencv::Error) -> Self {
        Error::OpenCV { source }
    }
}
