//!
//! very simple program to demonstrate color selection
//!
//! ```cargo run --example color-selection```
//!
use cv::prelude::*;
use std::{
    env,
    error::Error,
    path::PathBuf,
    time::Duration,
    sync::{Arc, RwLock},
};


fn main() -> Result<(), Box<dyn Error>> {
    let image_path = {
        let name = env::args().nth(1).unwrap_or("colors.png".to_string());
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base.join("assets").join(name)
    };

    let gui = cv::GUI::new("color selection");
    let offsets = color_range_offsets(&gui);

    let click_events = gui.mouse_events_for::<cv::MouseLeftBtnDown>();

    let mut color = cv::HSV::unsafe_new(0, 255, 51);
    loop {

        let mut image = cv::imread(&image_path)?;
        while let Ok(event) = click_events.try_recv() {
            color = image.at(&event.point())?;
            println!("HSV: {}, RGB: {}", color, cv::RGB::from(color));
        }

        let offsets = offsets.read().unwrap();
        let color_range = cv::HSVRange::from_hsv(&color, *offsets)?;
        let mut masked = image.convert_color().in_range(&color_range);
        let contours = masked.find_contours();
        image.draw_contours(&contours, cv::RGB::red(), 2);

        gui.show_for(&image, Duration::from_millis(100))?;
    }
}


fn color_range_offsets(gui: &cv::GUI) -> Arc<RwLock<(i32, i32, i32)>> {
    let offsets = Arc::new(RwLock::new((1, 1, 1)));
    std::thread::spawn({
        let h = gui.slider(&"Farbwert (H)", 1, 255);
        let s = gui.slider("Sättingung (S)", 1, 255);
        let v = gui.slider("Dunkelstufe (V)", 1, 255);
        let offsets = offsets.clone();
        move || {
            loop {
                while let Ok(hv) = h.recv_timeout(Duration::from_millis(100)) {
                    (*offsets.write().unwrap()).0 = hv;
                }
                while let Ok(sv) = s.recv_timeout(Duration::from_millis(100)) {
                    (*offsets.write().unwrap()).1 = sv;
                }
                while let Ok(vv) = v.recv_timeout(Duration::from_millis(100)) {
                    (*offsets.write().unwrap()).2 = vv;
                }
            }
        }
    });
    offsets
}
