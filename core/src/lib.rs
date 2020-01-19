use std::error::Error;

pub use cv::*;
pub mod err;
pub use err::*;
pub mod utils;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn find_puck(frame: &Mat<HSV>, hsv_range: &HSVRange) -> Result<Contours> {
    let mut masked = frame.in_range(hsv_range.min(), hsv_range.max());
    let contours = masked.find_contours()?;
    Ok(contours
        .iter()
        .filter(|c| {
            let area = c.area().unwrap();
            area > 350.0 && area < 3000.0
        })
        .collect())
}

pub fn predict(p1: &Point, p2: &Point, x: i32) -> Option<Point> {
    if p1.dist(&p2) < 0.2 {
        None
    } else {
        let m = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32;
        let n = p1.y as f32 - p1.x as f32 * m;
        println!("m: {}, n: {}", m, n);

        Some(Point {
            x,
            y: (n + x as f32 * m) as i32,
        })
    }
}

#[test]
pub fn test_predit() {
    assert_eq!(
        predict(&Point::new(127, 144), &Point::new(254, 213), 300),
        Some(Point::new(300, 237))
    );
}
