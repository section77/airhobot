use airhobot::*;
use cv::prelude::*;
use env_logger::{Builder, Env};
use log::*;
use std::{env, error, time::Duration, thread};
use structopt::StructOpt;
use airhobot::prelude::*;

mod args;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Play,
    Pause,
    PickColor,
}

fn main() -> Result<()> {
    let args = args::Args::from_args();
    Builder::from_env(Env::default().default_filter_or("info")).init();

//    let input = match args

    let gui = cv::GUI::new("AirHoBot - Detect Pucks");

    let mouse_events = gui.mouse_events();

    let blur_size = gui.sliderX("Blur size", 2, 0..15);
    let h_range_offset = gui.sliderX("H Offset", 35, 0..150);
    let s_range_offset = gui.sliderX("S Offset", 50, 0..150);
    let v_range_offset = gui.sliderX("V Offset", 75, 0..150);
    let mut hsv = cv::HSV::unsafe_new(70, 150, 100);

    use Mode::*;
    let mut mode = Play;

    for mut frame_clean in args.source()? {
//    let mut frame_clean = video.grab()?;
//    loop {
        //        let mut frame = cv::imread("assets/bild.png")?; // frame_clean.clone();
        let mut frame = frame_clean.clone();
        frame.blur(*blur_size.read().unwrap());
        match &mode {
            Play => {
                // if let Ok(next) = video.grab() {
                //     frame_clean = next;
                // } else {
                //     break;
                // }
            },
            PickColor => {
                use cv::gui::MouseEvent::*;
                while let Ok(event) = mouse_events.try_recv() {
                    match event {
                        LeftBtnDown(p) | LeftBtnUp(p) => {
                            frame.draw_rect(&cv::Rect::center(&p, 20, 20), cv::RGB::red(), 2);
                            hsv = frame.at(&p)?;
                            info!("selected hsv: {:?}", hsv);
                            gui.show_for(&frame, Duration::from_millis(1000))?;
                        },
                        _ => (),
                    }
                }
            }
            Pause => (),
        };


        let hsv_range = cv::HSVRange::from_hsv(&hsv,
                                               (*h_range_offset.read().unwrap(),
                                                *s_range_offset.read().unwrap(),
                                                *v_range_offset.read().unwrap()))?;
        debug!("hsv_range: {:?}", hsv_range);
        frame.draw_text(&format!("hsv: {}, range: {}", hsv, hsv_range), &cv::Point::new(5, 20), 0.5, cv::RGB::white(), 1);
        // find(&frame.to_hsv()?, &hsv_range, 550.0, 3000.0)?
        // .iter()
        //     .for_each(|c| frame.draw_rect(&c.bounding_rect(), cv::RGB::green(), 2));


        Puck::locate(&mut frame, &hsv_range);


        // show the actual frame
        if let Some(next) = match gui.show_for(&frame, Duration::from_millis(1000))? {
            ' ' => Some(if mode == Play { Pause } else { Play }),
            'c' => Some(PickColor),
            _ => None,
        } {
            mode = next;
        }
    }
    Ok(())
}
