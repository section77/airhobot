use airhobot::*;
use env_logger::{Builder, Env};
use log::*;
use opencv::highgui;
use std::{env, error, net::UdpSocket, time::Duration};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let win_name = "AiHoBot - test";
    let gui = GUI::new(win_name);

    let mut offset = (50, 50, 50);
    highgui::create_trackbar("Hue Offset", win_name, &mut offset.0, 100, None)?;
    highgui::create_trackbar("Saturation Offset", win_name, &mut offset.1, 100, None)?;
    highgui::create_trackbar("Value Offset", win_name, &mut offset.2, 100, None)?;

    let socket = UdpSocket::bind("192.168.1.220:6789")?;

    let mut puck_c_old: Option<cv::Point> = None;
    let mut cam = Cam::new_for_device_id(2)?;
    loop {
        let mut frame = cam.grab()?;
        let hsv_frame = frame.to_hsv()?;

        let puck_range = HSVRange::new(40..90, 70..120, 80..145)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));

        // let pusher_range =
        //     HSVRange::new(90..179, 60..180, 80..120)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));
        let pusher_range =
            HSVRange::new(0..30, 180..254, 80..190)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));

        // mark all pucks
        find_puck(&hsv_frame, &puck_range)?.iter().for_each(|pc| {
            frame.draw_rect(&pc.bounding_rect().unwrap(), RGB::green(), 2).unwrap();
        });
        // mark all pushers
        // find_puck(&hsv_frame, &pusher_range)?.iter().for_each(|pc| {
        //     frame
        //         .draw_rect(&pc.bounding_rect().unwrap(), RGB::new(255, 165, 0), 2)
        //         .unwrap();
        // });

        // green puck
        if let Some(pc) = find_puck(&hsv_frame, &puck_range)?.iter().next() {
            let puck_c = pc.center().unwrap();
            let text = format!("x: {}, y: {}", puck_c.x, puck_c.y);
            frame.draw_text(&text, &puck_c, 1.0, RGB::white(), 1).unwrap();
            frame.draw_rect(&pc.bounding_rect().unwrap(), RGB::green(), 4).unwrap();

            if let Some(old) = &puck_c_old {
                // pusher
                if let Some(pusher) = find_puck(&hsv_frame, &pusher_range)?.iter().next() {
                    let pusher_c = pusher.center().unwrap();
                    let text = format!("x: {}, y: {}", pusher_c.x, pusher_c.y);
                    frame.draw_text(&text, &pusher_c, 1.0, RGB::white(), 1).unwrap();
                    frame
                        .draw_rect(&pusher.bounding_rect().unwrap(), RGB::new(255, 165, 0), 2)
                        .unwrap();

                    let y = 405;
                    let x = if puck_c.x != old.x {
                        let m = (puck_c.y - old.y) / (puck_c.x - old.x);
                        if m == 0 || (old.y - puck_c.y).abs() < 5 {
                            // show the actual frame
                            gui.show_for(&frame, Duration::from_millis(10));
                            continue;
                        }
                        (y - puck_c.y) / m + puck_c.x
                    } else {
                        puck_c.x
                    };

                    let m1 = (x + y) as f32 * 2.22;
                    let m2 = (x - y) as f32 * 2.22;
                    // let m1 = (puck_c.x + puck_c.y) as f32 * 2.22;
                    // let m2 = (puck_c.x - puck_c.y) as f32 * 2.22;

                    let tx = (m1 - ((pusher_c.x + pusher_c.y) as f32 * 2.22)) as i32;
                    let ty = (m2 - ((pusher_c.x - pusher_c.y) as f32 * 2.22)) as i32;

                    let payload = format!("PRED: x: {}, y: {}", x, y);
                    frame
                        .draw_text(&payload, &Point::new(20, 80), 0.5, RGB::white(), 1)
                        .unwrap();

                    println!("target x: {}, target y: {}", tx, ty);

                    let payload = format!("{}:{}", tx, ty);
                    frame
                        .draw_text(&payload, &Point::new(20, 20), 0.5, RGB::white(), 1)
                        .unwrap();
                    if tx.abs() + ty.abs() > 50 {
                        socket
                            .send_to(payload.as_bytes(), "192.168.1.100:6789")
                            .expect("couldn't send data");
                        std::thread::sleep(Duration::from_millis(100))
                    }
                } else {
                    println!("pusher not found");
                }
            }
            puck_c_old = Some(puck_c);
        } else {
            println!("puck not found");
        }
        // show the actual frame
        gui.show_for(&frame, Duration::from_millis(10));
    }
}
