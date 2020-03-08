//!
//! very simple program to demonstrate object detection
//!
//! click with the mouse to see each step
//! left button: step forward, right button: step back
//!
//! ```cargo run --example detect-object```
//!
use cv::prelude::*;
use std::{
    env,
    error::Error,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    let image_path = {
        let name = env::args().nth(1).unwrap_or("objects.png".to_string());
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base.join("assets").join(name)
    };

    let gui = cv::GUI::new("detect object");
    let step: Arc<AtomicUsize> = {
        let step = Arc::new(AtomicUsize::new(0));
        thread::spawn({
            let step = step.clone();
            let mouse_events = gui.mouse_events();
            move || {
                while let Ok(event) = mouse_events.recv() {
                    let n = step.load(Ordering::Relaxed);
                    match event {
                        MouseEvent::LeftBtnDown(_) if n < 6 => step.fetch_add(1, Ordering::Relaxed),
                        MouseEvent::RightBtnDown(_) if n >= 1 => step.fetch_sub(1, Ordering::Relaxed),
                        _ => 0,
                    };
                }
            }
        });
        step
    };

    let blue_color_range = cv::HSVRange::new(100..=140, 240..=255, 200..=210)?;
    let green_color_range = cv::HSVRange::new(50..=60, 240..=255, 200..=210)?;
    let mut color_range = &blue_color_range;
    loop {
        let mut image = cv::imread(&image_path)?;
        let step = step.load(Ordering::Relaxed);

        image = match step {
            1 => {
                let masked = image.convert_color().in_range(&color_range);
                masked.convert_color()
            }
            2 => {
                let mut masked = image.convert_color().in_range(&color_range);
                let contours = masked.find_contours();
                masked.draw_contours(&contours, cv::RGB::new(128, 128, 128), 2);
                masked.convert_color()
            }
            3 => {
                let mut masked = image.convert_color().in_range(&color_range);
                let contours = masked.find_contours();
                image.draw_contours(&contours, cv::RGB::red(), 2);
                image
            }
            4 => {
                let mut masked = image.convert_color().in_range(&color_range);
                let contours = masked.find_contours();
                let contours = contours.iter().filter(|c| c.area() >= 10000.0).collect();
                image.draw_contours(&contours, cv::RGB::red(), 2);
                image
            }
            5 => {
                let mut masked = image.convert_color().in_range(&color_range);
                let contours = masked.find_contours();
                contours
                    .iter()
                    .filter(|c| {
                        if c.area() < 10000.0 {
                            return false;
                        }
                        let vertices = c.approx_poly_dp(0.02 * c.arc_length(true), true);
                        let vertices_n = vertices.len();

                        image.draw_text(&format!("{}", vertices_n), &c.center(), 1.0, cv::RGB::white(), 1);
                        image.draw_contours(&cv::Contours::new(vertices), cv::RGB::red(), 4);
                        vertices_n > 10
                    })
                    .for_each(drop);
                image
            }
            6 => {
                let mut masked = image.convert_color().in_range(&color_range);
                let contours = masked.find_contours();
                let contours = contours
                    .iter()
                    .filter(|c| {
                        let vertices = c.approx_poly_dp(0.02 * c.arc_length(true), true);
                        c.area() >= 10000.0 && vertices.len() >= 8
                    })
                    .collect();
                image.draw_contours(&contours, cv::RGB::red(), 2);
                image
            }
            _ => image,
        };

        let desc = match step {
            0 => "Original",
            1 => "Filter ueber Farbe",
            2 => "Konturen markiert",
            3 => "Original + Konturen",
            4 => "Filter ueber Groesse",
            5 => "Anzahl Kanten",
            6 => "Farbe+Groesse+Kanten",
            _ => "",
        };
        image.draw_text(&desc, &cv::Point::new(10, 540), 1.0, cv::RGB::new(128, 128, 128), 1);

        match gui.show_for(&image, Duration::from_millis(500))? {
            '1' => color_range = &blue_color_range,
            '2' => color_range = &green_color_range,
            'q' => return Ok(()),
            _ => (),
        };
    }
}
