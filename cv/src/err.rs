use std::{error, fmt};

#[derive(Debug)]
pub struct CVErr {
    pub component: Component,
    pub msg: String,
}

impl CVErr {
    pub fn new(component: Component, msg: String) -> Self {
        CVErr { component, msg }
    }
}

impl fmt::Display for CVErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cv error - component: {:?}, msg: {}", self.component, self.msg)
    }
}

impl error::Error for CVErr {}

#[derive(Debug, PartialEq)]
pub enum Component {
    VideoCapture,
    VideoWriter,
}
