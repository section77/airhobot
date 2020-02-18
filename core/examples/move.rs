use airhobot::*;
use env_logger::{Builder, Env};
use log::*;
use opencv::highgui;
use std::{env, error, net::UdpSocket, time::Duration,
          sync::{Mutex, Arc},

};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let win_name = "AiHoBot - test";
    let gui = GUI::new(win_name);

    let mut offset = (50, 50, 50);
    highgui::create_trackbar("Hue Offset", win_name, &mut offset.0, 100, None)?;
    highgui::create_trackbar("Saturation Offset", win_name, &mut offset.1, 100, None)?;
    highgui::create_trackbar("Value Offset", win_name, &mut offset.2, 100, None)?;

    let pusher_range =
        HSVRange::new(0..=50, 200..=255, 80..=180)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));


    let socket = UdpSocket::bind("192.168.1.220:6789")?;

    let mut start_p = Arc::new(Mutex::new(None));
    let mut end_p = Arc::new(Mutex::new(None));
    let mut send_done = false;



    highgui::set_mouse_callback(win_name, Some(Box::new({
        let start_p = start_p.clone();
        let end_p = end_p.clone();
        move |event, x, y, flags| {
            if event == highgui::EVENT_LBUTTONDOWN {
                let mut start_p = start_p.lock().unwrap();
                let mut end_p = end_p.lock().unwrap();
                if start_p.is_none() {
                    *start_p = Some(Point::new(x, y));
                } else if end_p.is_none() {
                    *end_p = Some(Point::new(x, y));
                } else {
                    *start_p = Some(Point::new(x, y));
                    *end_p = None;
                }

            }
        }
    })))?;


    let mut cam = Cam::new_for_device_id(2)?;
    loop {
        let mut frame = cam.grab()?;
        frame.blur(5);

        let mut hsv_frame = frame.to_hsv()?;


        if let Some(pusher) = find_puck(&hsv_frame, &pusher_range)?.iter().next() {
            let pusher_c = pusher.center().unwrap();
            frame
                .draw_rect(&pusher.bounding_rect().unwrap(), RGB::new(255, 165, 0), 2)
                .unwrap();

            if let Some(start_p) = &*start_p.lock().unwrap() {
                frame.draw_circle(start_p, 5, RGB::red(), 2);
                println!("start_p: {:?}", start_p);

                if let Some(end_p) = &*end_p.lock().unwrap() {
                    frame.draw_circle(end_p, 5, RGB::red(), 5);
                    println!("end_p: {:?}", end_p);

                    frame.draw_line(start_p, end_p, RGB::red(), 2);
                    let y = 350;
                    let x = if end_p.x != start_p.x {
                        let m = (end_p.y - start_p.y) as f32 / (end_p.x - start_p.x) as f32;
                        ((y - end_p.y) as f32 / m + end_p.x as f32) as i32
                    } else {
                        end_p.x
                    };
                    frame.draw_line(end_p, &Point::new(x, y), RGB::green(), 2);

                    if !send_done {
                        let m1 = (x + y) as f32 * 2.22;
                        let m2 = (x - y) as f32 * 2.22;

                        let tx = (m1 - ((pusher_c.x + pusher_c.y) as f32 * 2.22)) as i32;
                        let ty = (m2 - ((pusher_c.x - pusher_c.y) as f32 * 2.22)) as i32;

                        let payload = format!("{}:{}", tx, ty);
                        socket
                            .send_to(payload.as_bytes(), "192.168.1.100:6789")
                            .expect("couldn't send data");

                        send_done = true;
                    }
                } else {
                    send_done = false;
                }
            }
        }


        // let puck_range = HSVRange::new(40..90, 70..120, 80..145)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));

        // // let pusher_range =
        // //     HSVRange::new(90..179, 60..180, 80..120)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));
        // let pusher_range =
        //     HSVRange::new(0..30, 180..254, 80..190)?.adjust((offset.0 - 50, offset.1 - 50, offset.2 - 50));

        // // mark all pucks
        // find_puck(&hsv_frame, &puck_range)?.iter().for_each(|pc| {
        //     frame.draw_rect(&pc.bounding_rect().unwrap(), RGB::green(), 2).unwrap();
        // });
        // // mark all pushers
        // // find_puck(&hsv_frame, &pusher_range)?.iter().for_each(|pc| {
        // //     frame
        // //         .draw_rect(&pc.bounding_rect().unwrap(), RGB::new(255, 165, 0), 2)
        // //         .unwrap();
        // // });

        // // green puck
        // if let Some(pc) = find_puck(&hsv_frame, &puck_range)?.iter().next() {
        //     let puck_c = pc.center().unwrap();
        //     let text = format!("x: {}, y: {}", puck_c.x, puck_c.y);
        //     frame.draw_text(&text, &puck_c, 1.0, RGB::white(), 1).unwrap();
        //     frame.draw_rect(&pc.bounding_rect().unwrap(), RGB::green(), 4).unwrap();

        //     if let Some(old) = &puck_c_old {
        //         // pusher
        //         if let Some(pusher) = find_puck(&hsv_frame, &pusher_range)?.iter().next() {
        //             let pusher_c = pusher.center().unwrap();
        //             let text = format!("x: {}, y: {}", pusher_c.x, pusher_c.y);
        //             frame.draw_text(&text, &pusher_c, 1.0, RGB::white(), 1).unwrap();
        //             frame
        //                 .draw_rect(&pusher.bounding_rect().unwrap(), RGB::new(255, 165, 0), 2)
        //                 .unwrap();

        //             let y = 405;
        //             let x = if puck_c.x != old.x {
        //                 let m = (puck_c.y - old.y) / (puck_c.x - old.x);
        //                 if m == 0 || (old.y - puck_c.y).abs() < 5 {
        //                     // show the actual frame
        //                     gui.show_for(&frame, Duration::from_millis(10));
        //                     continue;
        //                 }
        //                 (y - puck_c.y) / m + puck_c.x
        //             } else {
        //                 puck_c.x
        //             };

        //             let m1 = (x + y) as f32 * 2.22;
        //             let m2 = (x - y) as f32 * 2.22;
        //             // let m1 = (puck_c.x + puck_c.y) as f32 * 2.22;
        //             // let m2 = (puck_c.x - puck_c.y) as f32 * 2.22;

        //             let tx = (m1 - ((pusher_c.x + pusher_c.y) as f32 * 2.22)) as i32;
        //             let ty = (m2 - ((pusher_c.x - pusher_c.y) as f32 * 2.22)) as i32;

        //             let payload = format!("PRED: x: {}, y: {}", x, y);
        //             frame
        //                 .draw_text(&payload, &Point::new(20, 80), 0.5, RGB::white(), 1)
        //                 .unwrap();

        //             println!("target x: {}, target y: {}", tx, ty);

        //             let payload = format!("{}:{}", tx, ty);
        //             frame
        //                 .draw_text(&payload, &Point::new(20, 20), 0.5, RGB::white(), 1)
        //                 .unwrap();
        //             if tx.abs() + ty.abs() > 50 {
        //                 socket
        //                     .send_to(payload.as_bytes(), "192.168.1.100:6789")
        //                     .expect("couldn't send data");
        //                 std::thread::sleep(Duration::from_millis(100))
        //             }
        //         } else {
        //             println!("pusher not found");
        //         }
        //     }
        //     puck_c_old = Some(puck_c);
        // } else {
        //     println!("puck not found");
        // }
        // show the actual frame
        gui.show_for(&frame, Duration::from_millis(10));
    }
}
