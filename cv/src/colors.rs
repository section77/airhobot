use super::ToScalar;
use rustcv::core::Scalar;

/// Represents a RGB color
#[derive(Debug, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB where {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
}

impl ToScalar for RGB where {
    fn to_scalar(&self) -> Scalar {
        Scalar {
            val1: self.b as f64,
            val2: self.g as f64,
            val3: self.r as f64,
            val4: 0.0,
        }
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

impl BGR where {
    pub fn new(b: u8, g: u8, r: u8) -> BGR {
        BGR { b, g, r }
    }
}

impl ToScalar for BGR where {
    fn to_scalar(&self) -> Scalar {
        Scalar {
            val1: self.b as f64,
            val2: self.g as f64,
            val3: self.r as f64,
            val4: 0.0,
        }
    }
}

impl From<RGB> for BGR {
    fn from(c: RGB) -> BGR {
        BGR { b: c.b, g: c.g, r: c.r }
    }
}

#[derive(Debug)]
pub struct HSV {
    h: u8,
    s: u8,
    r: u8,
}

impl HSV where {
    pub fn new(h: u8, s: u8, r: u8) -> Result<HSV, String> {
        match (h, s, r) {
            _ if h > 179 => Err(format!("invalid 'hue' - valid: 0 - 179, given: {}", h)),
            _ => Ok(HSV { h, s, r }),
        }
    }

    pub fn unsafe_new(h: u8, s: u8, r: u8) -> HSV {
        HSV { h, s, r }
    }
}

impl ToScalar for HSV {
    fn to_scalar(&self) -> Scalar {
        Scalar {
            val1: self.h as f64,
            val2: self.s as f64,
            val3: self.r as f64,
            val4: 0.0,
        }
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
