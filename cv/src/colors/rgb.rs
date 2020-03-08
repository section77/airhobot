use crate::*;
use opencv::core::{Scalar as OpencvScalar, Vec3b};
use std::fmt;

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

impl From<HSV> for RGB {
    fn from(hsv: HSV) -> RGB {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
        let h = hsv.h as f32 * 2.0;
        let s = hsv.s as f32 / 255.0;
        let v = hsv.v as f32 / 255.0;

        let c = v * s;
        let h = h / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0));
        let (r, g, b) =
            if h <= 1.0 {
                (c, x, 0.0)
            } else if h <= 2.0 {
                (x, c, 0.0)
            } else if h <= 3.0 {
                (0.0, c, x)
            } else if h <= 4.0 {
                (0.0, x, c)
            } else if h <= 5.0 {
                (x, 0.0, c)
            } else if h <= 6.0 {
                (c, 0.0, x)
            } else {
                (0.0, 0.0, 0.0)
            };

        let m = v - c;
        let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
        RGB::new(r as u8, g as u8, b as u8)
    }
}

impl From<Vec3b> for RGB {
    fn from(v: Vec3b) -> RGB {
        RGB::new(v[2], v[1], v[0])
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.r, self.g, self.b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hsv() {
        let hsv = HSV::unsafe_new(105, 255, 200);
        let rgb = RGB::new(0, 100, 200);
        assert_eq!(RGB::from(hsv), rgb);
    }

    #[test]
    fn hsv_rgb() {
        let checks = vec![
            RGB::new(0, 0, 0),
            RGB::new(128, 128, 128),
            RGB::new(255, 255, 255),
        ];
        checks.into_iter().for_each(|rgb| {
            let hsv = HSV::from(rgb);
            assert_eq!(RGB::from(hsv), rgb);
        });
    }
}
