use crate::*;
use opencv::core::{Scalar as OpencvScalar, Vec3b};

/// Represents a RGB color
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RGB {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}

impl ToOpencvScalar for RGB {
    fn to_opencv_scalar(&self) -> OpencvScalar {
        OpencvScalar::new(self.b as f64, self.g as f64, self.r as f64, 0.0)
    }
}

impl From<BGR> for RGB {
    fn from(c: BGR) -> RGB {
        RGB { r: c.r, g: c.g, b: c.b }
    }
}

impl From<Vec3b> for RGB {
    fn from(v: Vec3b) -> RGB {
        RGB::new(v[2], v[1], v[0])
    }
}
