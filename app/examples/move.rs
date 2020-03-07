use airhobot::prelude::*;
use cv::gui::*;
use env_logger::{Builder, Env};
use log::*;
use std::{
    env, error,
    net::UdpSocket,
    sync::{Arc, Mutex},
    time::Duration,
};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let win_name = "AirHoBot - test";
    let gui = cv::GUI::new(win_name);

    let mut offset = (50, 50, 50);

    let pusher_range = cv::HSVRange::new(0..=50, 200..=255, 80..=180)?;

    //    let socket = UdpSocket::bind("192.168.1.220:6789")?;

    let mut start_p = None;
    let mut end_p = None;
    let mut send_done = false;

    let mouse_events = gui.mouse_events();
    //    let mut cam = Source::cam(2)?;
    let mut cam = Source::image(asset_path("airhockey.png"))?;

    // board bounds
    let mut points = Vec::new();
    let mut frame = cam.next().unwrap();
    while points.len() < 4 {
        match mouse_events.try_recv() {
            Ok(MouseEvent::LeftBtnDown(p)) => {
                points.push(p.clone());
                println!("point: {:?}", p);
                frame.draw_circle(&p, 5, cv::RGB::red(), 5);
            }
            _ => (),
        };
        gui.show_for(&frame, Duration::from_millis(10));
    }
    let field = Field::new(points.clone());

    loop {
        let mut frame = cam.next().unwrap();
        frame.blur(5);

        let mut hsv_frame = frame.to_hsv()?;

        // draw board bounds
        field.draw(&mut frame);

        // puck abfragen
        match mouse_events.try_recv() {
            Ok(MouseEvent::LeftBtnDown(p)) => {
                println!("point set");
                frame.draw_circle(&p, 5, cv::RGB::green(), 5);
                if start_p.is_none() {
                    start_p = Some(p);
                } else if end_p.is_none() {
                    end_p = Some(p);
                } else {
                    start_p = None;
                    end_p = None;
                }
            }
            _ => (),
        };
        //        gui.show_for(&frame, Duration::from_millis(10));

        if let Some(pusher) = find(&hsv_frame, &pusher_range, 600.0, 3000.0)?.iter().next() {
            let pusher_c = pusher.center();
            frame.draw_rect(&pusher.bounding_rect(), cv::RGB::new(255, 165, 0), 2);

            if let Some(start_p) = start_p {
                frame.draw_circle(&start_p, 5, cv::RGB::red(), 2);

                if let Some(end_p) = end_p {
                    frame.draw_circle(&end_p, 5, cv::RGB::red(), 5);

                    frame.draw_line(&start_p, &end_p, cv::RGB::red(), 2);
                    let m = -(points[2].y() - points[3].y()) as f32 / (points[2].x() - points[3].x()) as f32;
                    let b = points[2].y() as f32 - points[3].x() as f32 * m;
                    let y = dbg!((m * ((points[2].x() + points[3].x()) / 2) as f32 + b) as i32 - 15);
                    let x = if end_p.x() != start_p.x() {
                        let m = (end_p.y() - start_p.y()) as f32 / (end_p.x() - start_p.x()) as f32;
                        ((y - end_p.y()) as f32 / m + end_p.x() as f32) as i32
                    } else {
                        end_p.x()
                    };
                    frame.draw_line(&end_p, &cv::Point::new(x, y), cv::RGB::green(), 2);

                    let (einschlag, bounce) = predict(&start_p, &end_p, y, &field);
                    frame.draw_circle(&einschlag, 8, cv::RGB::green(), 5);
                    if let Some(b) = bounce {
                        frame.draw_circle(&b, 8, cv::RGB::red(), 5);
                    }

                    //     if false { // !send_done {
                    //         let m1 = (x + y) as f32 * 2.22;
                    //         let m2 = (x - y) as f32 * 2.22;

                    //         let tx = (m1 - ((pusher_c.x() + pusher_c.y()) as f32 * 2.22)) as i32;
                    //         let ty = (m2 - ((pusher_c.x() - pusher_c.y()) as f32 * 2.22)) as i32;

                    //         let payload = format!("{}:{}", tx, ty);
                    //         socket
                    //             .send_to(payload.as_bytes(), "192.168.1.100:6789")
                    //             .expect("couldn't send data");

                    //         send_done = true;
                    //     }
                    // } else {
                    //     send_done = false;
                    // }
                }
            }

            gui.show_for(&frame, Duration::from_millis(10));
        }
    }
}
struct Field {
    lt: cv::Point,
    rt: cv::Point,
    lb: cv::Point,
    rb: cv::Point,
}

impl Field {
    pub fn new(vec: Vec<cv::Point>) -> Self {
        Field {
            lt: vec[0],
            rt: vec[1],
            rb: vec[2],
            lb: vec[3],
        }
    }

    pub fn draw<T>(&self, frame: &mut cv::Mat<T>) -> std::result::Result<(), Box<dyn error::Error>> {
        frame.draw_line(&self.lt, &self.rt, cv::RGB::red(), 2);
        frame.draw_line(&self.rt, &self.rb, cv::RGB::red(), 2);
        frame.draw_line(&self.rb, &self.lb, cv::RGB::red(), 2);
        frame.draw_line(&self.lb, &self.lt, cv::RGB::red(), 2);
        Ok(())
    }

    pub fn puck_bounces_side(&self, puck_pos: &cv::Point) -> Option<(cv::Point, cv::Point)> {
        if puck_pos.x() < self.lt.x() {
            Some((self.lt, self.lb))
        } else if puck_pos.x() > self.rb.x() {
            Some((self.rt, self.rb))
        } else {
            None
        }
    }
}

fn predict(s: &cv::Point, e: &cv::Point, y: i32, field: &Field) -> (cv::Point, Option<cv::Point>) {
    if e.x() == s.x() {
        return (cv::Point::new(e.x(), y), None);
    }

    let m = (e.y() - s.y()) as f32 / (e.x() - s.x()) as f32;
    let x = ((y - e.y()) as f32 / m + e.x() as f32) as i32;

    let point = cv::Point::new(x, y);

    if let Some((l_start, l_end)) = field.puck_bounces_side(&point) {
        let m1 = (e.y() - s.y()) as f32 / (e.x() - s.x()) as f32;
        let m2 = (l_end.y() - l_start.y()) as f32 / (l_end.x() - l_start.x()) as f32;

        let b1 = s.y() as f32 - s.x() as f32 * m1;
        let b2 = l_start.y() as f32 - l_start.x() as f32 * m2;

        let x = (b2 - b1) / (m1 - m2);
        let y_ = (m1 * x + b1) as i32;
        let bp = cv::Point::new(x as i32, y_ as i32);
        let point = cv::Point::new(((y - bp.y()) as f32 / (-m) + bp.x() as f32) as i32, y);
        (point, Some(bp))
    } else {
        (point, None)
    }
}

pub fn find(frame: &cv::Mat<cv::HSV>, hsv_range: &cv::HSVRange, area_min: f64, area_max: f64) -> Result<cv::Contours> {
    let mut masked = frame.in_range(hsv_range);
    let contours = masked.find_contours();
    Ok(contours
        .iter()
        .filter(|c| {
            let area = c.area();
            area > area_min && area < area_max
        })
        .collect())
}
