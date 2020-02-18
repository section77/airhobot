use opencv::core::Scalar as OpencvScalar;

pub trait ToOpencvScalar {
    fn to_opencv_scalar(&self) -> OpencvScalar;
}

/// Represents a RGB color
#[derive(Debug, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

/// Represents a BGR color
#[derive(Debug, PartialEq)]
pub struct BGR {
    b: u8,
    g: u8,
    r: u8,
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

#[derive(Debug, PartialEq)]
pub struct HSV {
    h: u8,
    s: u8,
    v: u8,
}

impl HSV {
    pub fn new(h: u8, s: u8, v: u8) -> Result<HSV, String> {
        match (h, s, v) {
            _ if h > 179 => Err(format!("invalid 'hue' value - valid: 0 - 179, given: {}", h)),
            _ => Ok(HSV { h, s, v }),
        }
    }

    pub fn unsafe_new(h: u8, s: u8, v: u8) -> HSV {
        HSV { h, s, v }
    }

    pub fn adjust(&self, offset: (i32, i32, i32)) -> HSV {
        let clamp = |min: i32, max: i32, v: i32| {
            let mut res = v;
            if res < min {
                res = min;
            }
            if res > max {
                res = max;
            }
            res
        };
        let h = clamp(0, 179, self.h as i32 + offset.0) as u8;
        let s = clamp(0, 254, self.s as i32 + offset.1) as u8;
        let v = clamp(0, 254, self.v as i32 + offset.2) as u8;

        HSV::unsafe_new(h, s, v)
    }
}

// impl From<RGB> for HSV {
//     fn from(rgb: RGB) -> HSV {
//  let (r, g, b) = (rgb.r as f32, rgb.g as f32, rgb.b as f32);
//         let (max, min, sep, coeff) = {
//             let (max, min, sep, coeff) = if r > g {
//                 (r, g, g - b, 0.0)
//             } else {
//                 (g, r, b - r, 2.0)
//             };
//             if b > max {
//                 (b, min, r - g, 4.0)
//             } else {
//                 let min_val = if b < min { b } else { min };
//                 (max, min_val, sep, coeff)
//             }
//         };

//         let mut h = 0.0;
//         let mut s = 0.0;
//         let v = max;

//         if max != min {
//             let d = max - min;
//             s = d / max;
//             h = ((sep / d) + coeff) * 60.0;
//         };

//  HSV::unsafe_new((h / 2.0) as u8, s as u8, v as u8)
//     }
// }

// impl From<RGB> for HSV {
//     fn from(rgb: RGB) -> HSV {

//  let (r, g, b) = (rgb.r as f32, rgb.g as f32, rgb.b as f32);
//  let min = r.min(g).min(b);
//  let max = r.max(g).max(b);

//  let mut out_h: f32;
//  let out_s: f32;
//  let out_v = max;

//  let delta = max - min;
//  if delta < 0.00001 {
//             out_s = 0.0f32;
//             out_h = 0.0f32;
//             return HSV::unsafe_new(out_h as u8, out_s as u8, out_v as u8);
//  }
//  if max > 0.0 {
//             out_s = delta / max;
//  } else {
//             out_s = 0.0;
//             out_h = 0.0;
//      return HSV::unsafe_new(out_h as u8, out_s as u8, out_v as u8);
//  }

//  if r >= max {
//             out_h = (g - b) / delta;
//  } else if g >= max {
//             out_h = 2.0 + (b - r) / delta;
//  } else {
//             out_h = 4.0 + (r - g) / delta;
//  }

//  out_h *= 60.0;

//  if out_h < 0.0 {
//             out_h += 360.0;
//  }

//  return HSV::unsafe_new((out_h / 2.0) as u8, out_s as u8, out_v as u8);
//     }
// }

// impl From<BGR> for HSV {
//     fn from(bgr: BGR) -> HSV {
//  HSV::from(RGB::from(bgr))
//     }
// }
impl ToOpencvScalar for HSV {
    fn to_opencv_scalar(&self) -> OpencvScalar {
        OpencvScalar::new(self.h as f64, self.s as f64, self.v as f64, 0.0)
    }
}

#[derive(Debug)]
pub struct HSVRange {
    min: HSV,
    max: HSV,
}
impl HSVRange {
    pub fn new(
        h_range: std::ops::RangeInclusive<u8>,
        s_range: std::ops::RangeInclusive<u8>,
        v_range: std::ops::RangeInclusive<u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(HSVRange {
            min: HSV::new(*h_range.start(), *s_range.start(), *v_range.start())?,
            max: HSV::new(*h_range.end(), *s_range.end(), *v_range.end())?,
        })
    }

    pub fn min(&self) -> &HSV {
        &self.min
    }

    pub fn max(&self) -> &HSV {
        &self.max
    }

    pub fn adjust(&self, offset: (i32, i32, i32)) -> Self {
        let min = self.min.adjust(offset);
        let max = self.max.adjust(offset);
        HSVRange { min, max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_from_bgr() {
        assert_eq!(RGB::from(BGR::new(1, 2, 3)), RGB::new(3, 2, 1));
    }

    #[test]
    fn bgr_from_rgb() {
        assert_eq!(BGR::from(RGB::new(1, 2, 3)), BGR::new(3, 2, 1));
    }
}
