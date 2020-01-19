use airhobot::*;
use env_logger::{Builder, Env};
use log::*;
use std::{env, error, time::Duration};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let gui = GUI::new("AirHoBot - Detect Pucks");

    let src_video_file = utils::asset_path(&env::args().nth(1).unwrap_or("airhockey-with-some-pucks.webm".into()));
    info!("load video from file: {:?}", src_video_file);
    let video = VideoCapture::open_file(&src_video_file)?;

    let out_video_file = src_video_file.with_extension("detect-pucks.avi");
    info!("write the result video to {:?}", out_video_file);
    let mut out = VideoWriter::new(&out_video_file, 15.0, 960, 544)?;

    for mut frame in video {
        let hsv_frame = frame.to_hsv()?;

        // green puck
        find_puck(&hsv_frame, &HSVRange::new(40..179, 100..255, 32..255)?)?
            .iter()
            .for_each(|c| frame.draw_rect(&c.bounding_rect().unwrap(), RGB::green(), 2).unwrap());

        // red puck
        find_puck(&hsv_frame, &HSVRange::new(0..6, 50..255, 50..255)?)?
            .iter()
            .for_each(|c| frame.draw_rect(&c.bounding_rect().unwrap(), RGB::red(), 2).unwrap());

        // orange puck
        find_puck(&hsv_frame, &HSVRange::new(10..50, 150..255, 110..255)?)?
            .iter()
            .for_each(|c| {
                frame
                    .draw_rect(&c.bounding_rect().unwrap(), RGB::new(255, 165, 0), 2)
                    .unwrap()
            });

        // write the actual frame in the output video
        out.write(&frame);

        // show the actual frame
        gui.show_for(&frame, Duration::from_millis(25));
    }
    Ok(())
}
