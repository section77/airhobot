use opencv::{prelude::Vector};

//pub mod cam;
pub mod colors;
pub mod contour;
pub mod error;
pub mod gui;
pub mod imageio;
pub mod mat;
pub mod point;
pub mod rect;
pub mod videoio;

pub mod prelude {
    pub use crate::mat::FindContours;
    pub use crate::mat::InRange;
    pub use crate::mat::ToHSV;
    pub use crate::gui::{MouseEvents, MouseEvent};
}

pub use crate::colors::*;
pub use crate::contour::*;
pub use crate::error::*;
pub use crate::gui::GUI;
pub use crate::gui::mouse_events::*;
pub use crate::imageio::imread;
pub use crate::mat::Mat;
pub use crate::point::*;
pub use crate::prelude::*;
pub use crate::rect::*;
pub use crate::videoio::{VideoCapture, VideoWriter};


type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum CVType {
    /// 8 bit unsigned, single channel
    CV8UC1 = 0,
    /// 8 bit signed, single channel
    CV8SC1 = 1,
    /// 8 bit unsigned, three channels
    CV8UC3 = 16,
    /// 8 bit signed, three channel
    CV8SC3 = 17,
}

impl CVType {
    fn unpack(&self) -> i32 {
        *self as i32
    }
}
