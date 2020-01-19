use std::{convert::From, error, fmt};

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

impl From<opencv::Error> for CVErr {
    fn from(err: opencv::Error) -> Self {
        CVErr::new(Component::Opencv, err.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum Component {
    Opencv,
    VideoCapture,
    VideoWriter,
}
