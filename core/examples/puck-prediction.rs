use airhobot::*;
use env_logger::{Builder, Env};
use log::*;
use std::{env, error, time::Duration};

fn main() -> Result<(), Box<dyn error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let gui = GUI::new("AirHoBot - Puck Prediction");

    let src_video_file = utils::asset_path(&env::args().nth(1).unwrap_or("airhockey-with-some-pucks.webm".into()));
    info!("load video from file: {:?}", src_video_file);
    let video = VideoCapture::open_file(&src_video_file)?;

    let mut path = Path::new();

    let mut p1 = Point::new(0, 0);
    let mut p2 = Point::new(0, 0);
    for mut frame in video {
        let hsv_frame = frame.to_hsv()?;

        // green puck
        find_puck(&hsv_frame, &HSVRange::new(67..179, 100..255, 32..255)?)?
            .iter()
            .for_each(|c|{

                frame.draw_rect(&c.bounding_rect().unwrap(), RGB::green(), 2).unwrap();
                let p = c.center().unwrap();
                path.push(&p);
                let text = format!("x: {}, y: {}", p.x, p.y);
                frame.draw_text(&text, &p, 2.0, RGB::white(), 1).unwrap();
                path.draw_path(&mut frame, RGB::red(), 2);
                p2 = p;
            });


        predict(&p1, &p2, 400)
            .iter()
            .for_each(|p| {
                println!("p1: {:?}, p2: {:?}, p: {:?}", p1, p2, p);
                if p.y > 0 {
                    frame.draw_line(&p1, &p, RGB::black(), 2).unwrap();
                }
            });

        // show the actual frame
        gui.show_for(&frame, Duration::from_millis(100));
        p1 = p2.clone();
    }
    Ok(())
}
