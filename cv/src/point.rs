use serde::{Deserialize, Serialize};
use std::{fmt, ops::Sub};

/// Point in an 2D environment
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub(crate) fn pack(p: opencv::core::Point) -> Self {
        Self::new(p.x, p.y)
    }

    // FIXME: pub(crate)
    pub fn unpack(&self) -> opencv::core::Point {
        opencv::core::Point_ { x: self.x, y: self.y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn dist(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    pub fn norm(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(mut self, rhs: Point) -> Self::Output {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self
    }
}
