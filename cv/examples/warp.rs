use cv::prelude::*;
use opencv::{calib3d, prelude::*};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = cv::imread("/home/j/prj/s77/airhobot/app/assets/airhockey.png")?;

    let gui = cv::GUI::new("AirHoBot");
    let mut mouse_events: Option<cv::MouseEvents> = None;

    loop {
        if let Some(mouse_events) = mouse_events.take() {
            let mut points = Vec::new();
            while points.len() < 4 {
                while let Ok(event) = mouse_events.try_recv() {
                    frame.draw_circle(&event.point(), 2, cv::RGB::red(), 2);
                    points.push(event.point());
                }
                gui.show_for(&frame, Duration::from_millis(100))?;
            }
            println!("points: {:?}", points);
            // let w = (points[0] - points[1]).norm().max((points[2] - points[3]).norm()) as i32;
            // let h = (points[1] - points[2]).norm().max((points[3] - points[0]).norm()) as i32;

            let w = points[1].x() - points[0].x();
            let h = points[2].y() - points[1].y();

            let mut dst_corners = opencv::types::VectorOfPoint::new();
            dst_corners.push(cv::Point::new(0, 0).unpack());
            dst_corners.push(cv::Point::new(w, 0).unpack());
            dst_corners.push(cv::Point::new(w, h).unpack());
            dst_corners.push(cv::Point::new(0, h).unpack());
            let roi_corners_mat = opencv::core::Mat::from_exact_iter(points.iter().map(|p| p.unpack()))?;
            let dst_corners_mat = opencv::core::Mat::from_exact_iter(dst_corners.iter())?;
            let hom = calib3d::find_homography(
                &roi_corners_mat,
                &dst_corners_mat,
                &mut opencv::core::Mat::default()?,
                0,
                3.,
            )?; //get homography
            let mut warped_image = opencv::core::Mat::default()?;
            let warped_image_size = opencv::core::Size::new(w, h);
            opencv::imgproc::warp_perspective(
                &frame.unpack(),
                &mut warped_image,
                &hom,
                warped_image_size,
                opencv::imgproc::INTER_LINEAR,
                opencv::core::BORDER_CONSTANT,
                opencv::core::Scalar::default(),
            )?; // do perspective transformation
            frame = cv::Mat::<cv::BGR>::pack(warped_image);
        }

        match gui.show_for(&frame, Duration::from_millis(100))? {
            '1' => mouse_events = Some(gui.mouse_events_for::<cv::MouseLeftBtnDown>()),
            'q' => break,
            _ => (),
        }
    }
    println!("hallo");

    Ok(())
}
