use super::*;
use rustcv::highgui::*;
use std::time;

/// Simple GUI
pub struct GUI {
    win: Window,
}

impl GUI where {
    pub fn new(name: &str) -> GUI {
        GUI {
            win: Window::new(name, WindowFlag::FreeRatio).unwrap(),
        }
    }

    // Show the given frame for the given duration
    pub fn show<T>(&self, mat: &Mat<T>, dur: time::Duration) {
        self.win.show(mat.to_rustcv());

        // dur.as_millis() is only in nightly
        let millis = dur.as_secs() * 1000_u64 + dur.subsec_millis() as u64;
        self.win.wait_key(i32::try_from(millis).expect("invalid millis"));
    }
}
