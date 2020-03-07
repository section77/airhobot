use crate::*;
use serde::{Deserialize, Serialize};
use std::{fmt, ops::RangeInclusive};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct HSVRange {
    min: HSV,
    max: HSV,
}

impl HSVRange {
    pub fn new(h_range: RangeInclusive<u8>, s_range: RangeInclusive<u8>, v_range: RangeInclusive<u8>) -> Result<Self> {
        Ok(HSVRange {
            min: HSV::new(*h_range.start(), *s_range.start(), *v_range.start())?,
            max: HSV::new(*h_range.end(), *s_range.end(), *v_range.end())?,
        })
    }

    pub fn from_hsv(hsv: &HSV, offsets: (i32, i32, i32)) -> Result<Self> {
        let min = hsv.adjust((-offsets.0, -offsets.1, -offsets.2));
        let max = hsv.adjust(offsets);

        Ok(HSVRange { min, max })
    }

    pub fn min(&self) -> &HSV {
        &self.min
    }

    pub fn max(&self) -> &HSV {
        &self.max
    }
}

impl fmt::Display for HSVRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.min(), self.max())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hsv() {
        let hsv = HSV::unsafe_new(40, 60, 80);
        let range = HSVRange::from_hsv(&hsv, (10, 20, 30)).unwrap();
        assert_eq!(range, HSVRange::new(30..=50, 40..=80, 50..=110).unwrap());
    }
}
