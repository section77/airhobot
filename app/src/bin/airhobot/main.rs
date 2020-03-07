use airhobot::prelude::*;
use log::{error, warn, info};
use snafu::ErrorCompat;
use std::{net::UdpSocket, time::Duration};
use structopt::StructOpt;

mod args;
mod control_panel;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                println!("{}", backtrace);
            }
        }
    }
}

fn run() -> Result<()> {
    let args = args::Args::from_args();
    init_logger(&args);

    // load the config
    let cfg = Config::load(&args.config_file).unwrap_or_else(|e| {
        error!("unable to load config: {} - start with default config", e);
        Config::default()
    });

    // initialize the control panel
    let (mut control_panel, cfg) = control_panel::ControlPanel::new(cfg);



    let gui = cv::GUI::new("AirHoBot");
    let mut state = {
        let frame = args.source()?.grab().ok_or::<Error>("empty source".into())?;
        State::new(cfg, frame)
    };


    let mut pause = false;
    for frame in args.source()? {
        state = state.next(frame);

        state.crop_frame()?;
        state.apply_filter_frame()?;

        let frame_hsv = state.frame.to_hsv()?;
        state.puck = {
            let puck_contours = detect("puck", &state.cfg.read()?.puck, &frame_hsv)?;
            state.frame.draw_contours(&puck_contours, cv::RGB::white(), 2);
            let puck = puck_contours.iter().next().map(|c| c.center());
            puck
        };

        state.pusher = {
            let pusher_contours = detect("pusher", &state.cfg.read()?.pusher, &frame_hsv)?;
            state.frame.draw_contours(&pusher_contours, cv::RGB::red(), 2);
            let pusher = pusher_contours.iter().next().map(|c| c.center());
            pusher
        };


        loop {
            control_panel.repaint()?;
            match gui.show_for(&state.frame, Duration::from_millis(args.delay))? {
                '1' => state.cfg.write()?.roi = select_field(&state, &gui)?,
                '2' => state.cfg.write()?.pusher.color = pick_color(&state, &gui)?,
                '3' => state.cfg.write()?.puck.color = pick_color(&state, &gui)?,
                '4' => simulate_puck(&state, &gui)?,
                '5' => move_pusher(&state, &gui)?,
                'c' => control_panel.toggle(),
                'f' => break, // next frame
                'r' => {
                    *state.cfg.write()? = Config::load(&args.config_file)?;
                    break; // next frame
                },
                's' => state.cfg.read()?.save(&args.config_file)?,
                'q' => return Ok(()),
                ' ' => pause = !pause,
                _ => (),
            }

            if !pause {
                break
            }
        }
    }
    Ok(())
}


fn select_field(state: &State, gui: &cv::GUI) -> Result<Roi> {
    info!("Select field - select left-top, right-top, right-bottom, left-bottom");
    let mouse_events = gui.mouse_events_for::<cv::MouseLeftBtnDown>();
    let mut frame = state.frame_orig.clone();
    let mut vec = Vec::new();
    while vec.len() < 4 {
        while let Ok(event) = mouse_events.try_recv() {
            frame.draw_circle(&event.point(), 4, cv::RGB::red(), 2);
            vec.push(event.point());
        }
        gui.show_for(&frame, Duration::from_millis(10))?;
    }
    Roi::from_vec(vec)
}


fn pick_color(state: &State, gui: &cv::GUI) -> Result<cv::HSV> {
    info!("Pick color");
    let mouse_events = gui.mouse_events_for::<cv::MouseLeftBtnDown>();
    let mut frame = state.frame.clone();
    loop {
        while let Ok(event) = mouse_events.try_recv(){
            frame.draw_rect(&cv::Rect::center(&event.point(), 10, 10), cv::RGB::red(), 2);
            gui.show_for(&frame, Duration::from_millis(1000))?;
            return Ok(frame.at_avg(&event.point(), 3)?);
        }
        gui.show_for(&frame, Duration::from_millis(10))?;
    }
}

fn simulate_puck(state: &State, gui: &cv::GUI) -> Result<()> {
    info!("Simulate puck - select two points");
    let mouse_events = gui.mouse_events_for::<cv::MouseLeftBtnDown>();
    let mut frame = state.frame.clone();
    let mut points = Vec::new();
    while points.len() < 2 {
        while let Ok(event) = mouse_events.try_recv() {
            frame.draw_circle(&event.point(), 4, cv::RGB::red(), 2);
            points.push(event.point());
        }
        gui.show_for(&frame, Duration::from_millis(10))?;
    }
    let (from, to) = (points[0], points[1]);
    frame.draw_line(&from, &to, cv::RGB::white(), 2);
    let path = predict(&frame, &from, &to);
    for (i, from) in path.iter().enumerate() {
        if let Some(to) = path.get(i + 1) {
            frame.draw_line(from, to, cv::RGB::red(), 2);
        }
    }
    gui.show_for(&frame, Duration::from_millis(2000))?;
    Ok(())
}

fn move_pusher(state: &State, gui: &cv::GUI) -> Result<()> {
    info!("Move pusher - select a target point");
    let mouse_events = gui.mouse_events_for::<cv::MouseLeftBtnDown>();
    let mut frame = state.frame.clone();
    loop {
        while let Ok(event) = mouse_events.try_recv(){
            let target = event.point();
            frame.draw_circle(&event.point(), 4, cv::RGB::red(), 2);

            if let Some(pusher) = &state.pusher {
                info!("pusher location: {:?}", pusher);
                info!("target location: {:?}", target);

                let factor = 2.22;
                let m1 = (target.x() + target.y()) as f32 * factor;
                let m2 = (target.x() - target.y()) as f32 * factor;

                let tx = (m1 - ((pusher.x() + pusher.y()) as f32 * factor)) as i32;
                let ty = (m2 - ((pusher.x() - pusher.y()) as f32 * factor)) as i32;
                let payload = format!("{}:{}", tx, ty);
                info!("send payload: {}", payload);
                let socket = UdpSocket::bind("0.0.0.0:6789")?;
                socket
                    .send_to(payload.as_bytes(), state.cfg.read()?.driver.addr)?;
                gui.show_for(&frame, Duration::from_millis(1000))?;
                return Ok(())
            } else {
                warn!("Pusher not found");
                return Ok(())
            }
        }
        gui.show_for(&frame, Duration::from_millis(10))?;
    }
}


fn init_logger(args: &args::Args) {
    let filter = {
        let name = env!("CARGO_PKG_NAME").replace("-", "_");
        match args.verbose {
            0 => format!("{}=info", name),
            1 => "debug".to_string(),
            _ => "trace".to_string(),
        }
    };
    env_logger::from_env(env_logger::Env::default().default_filter_or(filter)).init();
}
