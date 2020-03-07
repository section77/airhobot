use crate::*;
use opencv::core::{Scalar as OpencvScalar, Vec3b};

/// Represents a BGR color
#[derive(Debug, PartialEq)]
pub struct BGR {
    pub(crate) b: u8,
    pub(crate) g: u8,
    pub(crate) r: u8,
}

impl BGR {
    pub fn new(b: u8, g: u8, r: u8) -> BGR {
        BGR { b, g, r }
    }
}

impl ToOpencvScalar for BGR {
    fn to_opencv_scalar(&self) -> OpencvScalar {
        OpencvScalar::new(self.b as f64, self.g as f64, self.r as f64, 0.0)
    }
}

impl From<RGB> for BGR {
    fn from(c: RGB) -> BGR {
        BGR { b: c.b, g: c.g, r: c.r }
    }
}

impl From<Vec3b> for BGR {
    fn from(v: Vec3b) -> BGR {
        BGR::new(v[0], v[1], v[2])
    }
}
