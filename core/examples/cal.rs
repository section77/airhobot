use airhobot::*;
use env_logger::{Builder, Env};
use log::*;
use opencv::highgui;
use std::{
    env,
    error,
    net::UdpSocket,
    time::Duration,
    sync::{Arc, Mutex},
};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut marker_p = Point::new(0, 0);
    let click_p = Arc::new(Mutex::new(None));

    let win_name = "AiHoBot - cal";
    let gui = GUI::new(win_name);
    opencv::highgui::set_mouse_callback(win_name, Some(Box::new({
	let click_p = click_p.clone();
	move |event, x, y, _flags| {
	    if event == opencv::highgui::EVENT_LBUTTONDOWN {
		*click_p.lock().unwrap() = Some(Point::new(x, y));
		println!("click at: {}, {}", x, y);
	    }
	}
    }))).unwrap();


//    let impath = utils::asset_path("bild.png");
    let mut cam = Cam::new_for_device_id(2)?;

    loop {
        //let mut frame = cv::imread::<RGB>(&impath)?;
        let mut frame = cam.grab()?;
        frame.blur(5);
        let hsv_frame = frame.to_hsv()?;

	{
	    let mut click_p = click_p.lock().unwrap();
	    if let Some(p) = click_p.take() {
//		*click_p = None;
		    marker_p = p.clone();
		    dbg!(frame.at_2d(p.x, p.y))?;
	    }
	}
	let rec = Rect::new(marker_p.x, marker_p.y, 10, 10);
	frame.draw_rect(&rec, RGB::green(), 2).unwrap();

        gui.show_for(&frame, Duration::from_millis(100));
    }
}
