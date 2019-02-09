use rustcv::core::Scalar;

pub trait ToScalar {
    fn to_scalar(&self) -> Scalar;
}

/// Represents a RGB color
#[derive(Debug, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB where {
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
            _ if h > 179 => Err(format!("invalid 'hue' value - valid: 0 - 179, given: {}", h)),
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

pub struct HSVRange {
    min: HSV,
    max: HSV,
}
impl HSVRange {
    pub fn new(
        h_range: std::ops::Range<u8>,
        s_range: std::ops::Range<u8>,
        v_range: std::ops::Range<u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(HSVRange {
            min: HSV::new(h_range.start, s_range.start, v_range.start)?,
            max: HSV::new(h_range.end, s_range.end, v_range.end)?,
        })
    }

    pub fn min(&self) -> &HSV {
        &self.min
    }

    pub fn max(&self) -> &HSV {
        &self.max
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
