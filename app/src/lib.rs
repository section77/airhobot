//!
//! Air-Hockey Robot
//!
//! ```
//! use airhobot::prelude::*;
//! ```
use log::{debug, trace};
use std::time::Duration;

mod config;
mod error;
mod predict;
//mod puck;
mod source;
mod state;
mod utils;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, Error>;

    pub use crate::config::*;
    pub use crate::detect;
    pub use crate::error::*;
    pub use crate::predict::predict;
  //  pub use crate::puck::*;
    pub use crate::source::*;
    pub use crate::state::State;
    pub use crate::utils::*;
    pub use crate::PuckSpeed;

    pub use cv::prelude::*;
}
use prelude::*;

pub fn detect<S>(what: S, detector: &Detector, frame: &cv::Mat<cv::HSV>) -> Result<cv::Contours>
where
    S: Into<String>,
{
    let what = what.into();

    // mask the colors which are not in the given color range
    let mut masked = frame.in_range(&detector.color_range()?);

    // find all countours in the masked frame
    let contours_all = masked.find_contours();

    // filter countours
    let contours: cv::Contours = contours_all
        .iter()
        .filter(|c| {
            // contour area size
            let area = c.area();

            // approx number of vertices for the polygon
            let vertices = c.approx_poly_dp(0.01 * c.arc_length(true), true);

            // check
            let matches = vertices.len() > detector.min_vertices
                && area > detector.area_range.start
                && area < detector.area_range.end;

            trace!(
                "{} detector - filter countours - area {}, area-range: {:?}, vertices: {}, min-vertices: {}",
                what,
                area,
                detector.area_range,
                vertices.len(),
                detector.min_vertices
            );

            matches
        })
        .collect();

    debug!(
        "{} detector - {} contours detected, {} countours matches the criteria",
        what,
        contours_all.len(),
        contours.len()
    );

    Ok(contours)
}

#[derive(Debug, PartialEq)]
pub enum PuckSpeed {
    Slow,
    Fast,
    Wormhole,
}
impl PuckSpeed {
    pub fn new(dur: Duration, dist: f64) -> Self {
        let speed = dbg!(dur.as_millis() as f64 / dist);
        if speed < 1.5 {
            PuckSpeed::Slow
        } else if speed < 2.0 {
            PuckSpeed::Fast
        } else {
            PuckSpeed::Wormhole
        }
    }
}

// pub fn predict(p1: &Point, p2: &Point, x: i32) -> Option<Point> {
//     if p1.dist(&p2) < 0.2 {
//         None
//     } else {
//         let m = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32;
//         let n = p1.y as f32 - p1.x as f32 * m;
//         println!("m: {}, n: {}", m, n);

//         Some(Point {
//             x,
//             y: (n + x as f32 * m) as i32,
//         })
//     }
// }

// #[test]
// pub fn test_predit() {
//     assert_eq!(
//         predict(&Point::new(127, 144), &Point::new(254, 213), 300),
//         Some(Point::new(300, 237))
//     );
// }
