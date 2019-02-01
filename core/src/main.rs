use airhobot::*;
use env_logger::{Builder, Env};
use log::*;
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> Result<(), AirHoBotErr>{
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut cam = Cam::new()?;
    let gui = GUI::new("AirHoBot");

    let socket = {
        let addr = "0.0.0.0:6789";
        info!("open udp socket {}", addr);
        UdpSocket::bind(addr)?
    };

    let pi_addr = "192.168.1.222:6789";
    info!("connect to pi@{}", pi_addr);
    socket.connect(pi_addr)?;
    info!("connected");

    loop {
        let mut frame: Mat<BGR> = cam.grab().ok_or(AppErr("no frame grabbed".to_string()))?;
        let masked = frame
            .to_hsv()
            .in_range(HSV::unsafe_new(90, 150, 80), HSV::unsafe_new(130, 255, 255));

        if let Some(p) = masked.find_center() {
            println!("center: {:?}", p);
            socket.send(&format!("{}:{}", p.x, p.y).into_bytes())?;

            frame = frame.apply_mask(&masked);
            frame.draw_circle(&p, 5, RGB::new(255, 0, 0), 5);
        }

        gui.show(&frame, Duration::from_millis(10));
    }
}

pub fn predict(p1: &Point, p2: &Point, x: i32) -> Option<Point> {
    if p1.dist(&p2) < 0.2 {
        None
    } else {
        let m = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32;
        let n = p1.y as f32 - p1.x as f32 * m;
        println!("m: {}, n: {}", m, n);

        Some(Point {
            x,
            y: (n + x as f32 * m) as i32,
        })
    }
}

#[test]
pub fn test_predit() {
    assert_eq!(
        predict(&Point::new(127, 144), &Point::new(254, 213), 300),
        Some(Point::new(300, 237))
    );
}
