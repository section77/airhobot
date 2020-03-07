use crate::*;
use opencv::core::Scalar as OpencvScalar;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct HSV {
    pub(crate) h: u8,
    pub(crate) s: u8,
    pub(crate) v: u8,
}

impl HSV {
    pub fn new(h: u8, s: u8, v: u8) -> Result<HSV> {
        match (h, s, v) {
            _ if h > 179 => Err(Error::UserInput {
                msg: format!("invalid 'hue' value - valid: 0 - 179, given: {}", h),
            }),
            _ => Ok(HSV { h, s, v }),
        }
    }

    pub fn unsafe_new(h: u8, s: u8, v: u8) -> HSV {
        HSV { h, s, v }
    }

    pub fn adjust(&self, offset: (i32, i32, i32)) -> HSV {
        let clamp = |min: i32, max: i32, v: i32| {
            if v < min {
                min as u8
            } else if v > max {
                max as u8
            } else {
                v as u8
            }
        };

        let h = clamp(0, 179, self.h as i32 + offset.0) as u8;
        let s = clamp(0, 255, self.s as i32 + offset.1) as u8;
        let v = clamp(0, 255, self.v as i32 + offset.2) as u8;

        HSV::unsafe_new(h, s, v)
    }
}

impl From<RGB> for HSV {
    fn from(rgb: RGB) -> HSV {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
        let r = rgb.r as f32 / 255.0;
        let g = rgb.g as f32 / 255.0;
        let b = rgb.b as f32 / 255.0;

        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let diff = max - min;

        let mut h = 60.0
            * if max == min {
                0.0
            } else if max == r {
                (g - b) / diff
            } else if max == g {
                2.0 + (b - r) / diff
            } else if max == b {
                4.0 + (r - g) / diff
            } else {
                unreachable!()
            };
        if h < 0.0 {
            h += 360.0;
        }

        let s = if max == 0.0 { 0.0 } else { diff / max };

        let h = (h / 2.0).max(0.0);
        HSV::unsafe_new(h as u8, (s * 255.0) as u8, (max * 255.0) as u8)
    }
}

impl ToOpencvScalar for HSV {
    fn to_opencv_scalar(&self) -> OpencvScalar {
        OpencvScalar::new(self.h as f64, self.s as f64, self.v as f64, 0.0)
    }
}

impl fmt::Display for HSV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.h, self.s, self.v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb() {
        let rgb = RGB::new(0, 100, 200);
        let hsv = HSV::from(rgb);
        assert_eq!(hsv, HSV::unsafe_new(105, 255, 200));
    }
}
