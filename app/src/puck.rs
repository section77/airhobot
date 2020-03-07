use crate::prelude::*;
use log::debug;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Puck {
    pos: cv::Point,
    path: VecDeque<cv::Point>,
}

impl Puck {
    pub fn locate(frame: &mut cv::Mat<cv::BGR>, range: &cv::HSVRange) {
        let area_min = 350.0;
        let area_max = 3000.0;

        let frame_hsv = frame.to_hsv().unwrap();

        // mask the colors which are not in the given color range
        let mut masked = frame_hsv.in_range(range);

        // find all countours in the masked frame
        let contours_all = masked.find_contours();

        // filter countours by their area size
        let contours: cv::Contours = contours_all
            .iter()
            .filter(|c| {
                let approx = c.approx_poly_dp(0.01 * c.arc_length(true), true);
                dbg!(approx.len());
                let area = c.area();
                true // approx.len() > 8// && area > area_min && area < area_max
            })
            .collect();
        debug!(
            "{} puck contours found - {} have a area between {} and {}",
            contours_all.len(),
            contours.len(),
            area_min,
            area_max
        );

        // mark all found contours
        frame.draw_contours(&contours, cv::RGB::white(), 2);
    }

    pub fn draw(&self, frame: &mut cv::Mat<cv::BGR>) {
        if let Some(p) = self.path.iter().last() {
            let rect = cv::Rect::center(&p, 10, 10);
            frame.draw_rect(&rect, cv::RGB::red(), 2);
        }
    }

    pub fn draw_path(&self, frame: &mut cv::Mat<cv::BGR>) {
        let color = cv::RGB::red();
        let thickness = 2;

        let mut iter = self.path.iter().peekable();
        for (from, to) in self.path.iter().zip(iter.peek()) {
            frame.draw_arrowed_line(&from, &to, &color, thickness);
        }
    }
}
