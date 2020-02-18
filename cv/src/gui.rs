use super::*;
use opencv::highgui;
use std::time;

/// Simple GUI
pub struct GUI {
    name: String,
}

impl GUI {
    pub fn new(name: &str) -> GUI {
        highgui::named_window(name, 1).unwrap();
        GUI { name: name.to_string() }
    }

    pub fn show<T>(&self, mat: &Mat<T>) {
        // FIXME: use max_value!
        self.show_for(mat, time::Duration::from_millis(50000)); // i32::max_value as u64))
    }

    // Show the given frame for the given duration
    pub fn show_for<T>(&self, mat: &Mat<T>, dur: time::Duration) {
        highgui::imshow(&self.name, &mat.unpack());

        let millis = dur.as_millis() as i32;
        highgui::wait_key(millis);
    }
}
