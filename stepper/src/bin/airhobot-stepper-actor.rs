use env_logger::{Builder, Env};
use log::*;
use std::error::Error;
use std::io;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use stepper::*;

fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .default_format_timestamp_nanos(true)
        .init();

    // listen on socket
    let socket = {
        let addr = "0.0.0.0:6789";
        info!("listen on {}", addr);
        UdpSocket::bind(addr)?
    };

    // setup stepper
    let stepper_l = Arc::new(Mutex::new(Stepper::new(
        "L",
        EnablePin(23),
        StepPin(27),
        DirectionPin(17),
    )?));

    let stepper_r = Arc::new(Mutex::new(Stepper::new(
        "R",
        EnablePin(24),
        StepPin(13),
        DirectionPin(5),
    )?));

    loop {
        // read from network socket
        let mut buf = [0; 1024];
        info!("waiting for data ...");
        let (len, _src) = socket.recv_from(&mut buf)?;
        let buf = String::from_utf8_lossy(&buf[..len]);

        // try to parse the telegram
        match split_fields(&buf) {
            Ok((l, r)) => {
                info!("trigger stepper actions");
                let thread_hndl_l = make_steps_async(stepper_l.clone(), l);
                let thread_hdnl_r = make_steps_async(stepper_r.clone(), r);

                // wait for stepper actions
                info!("wait for steppers");
                thread_hndl_l.join().unwrap();
                thread_hdnl_r.join().unwrap();
                info!("done");
            }
            Err(err) => error!("invalid telegram: '{}' - {} - ignore telegram", buf, err),
        }
    }
}

fn split_fields(s: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let s = s.trim();
    match s.split_terminator(':').collect::<Vec<_>>().as_slice() {
        [l, r] => {
            let l = l.parse()?;
            let r = r.parse()?;
            Ok((l, r))
        }
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "no / to many split terminator(s) ':' found",
        ))),
    }
}

fn make_steps_async(stepper: Arc<Mutex<Stepper>>, x: i32) -> thread::JoinHandle<()> {
    let direction = if x < 0 { Direction::Left } else { Direction::Right };

    let steps = x.abs() as u32;

    thread::spawn(move || {
        stepper.lock().unwrap().step_n(direction, steps);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_fields_normal_numbers() {
        let (l, r) = split_fields("4:5").unwrap();
        assert_eq!(l, 4);
        assert_eq!(r, 5);
    }

    #[test]
    fn split_fields_prefixed_numbers() {
        let (l, r) = split_fields("+3:-6").unwrap();
        assert_eq!(l, 3);
        assert_eq!(r, -6);
    }

    #[test]
    fn split_fields_with_newline() {
        let (l, r) = split_fields("2:7\n").unwrap();
        assert_eq!(l, 2);
        assert_eq!(r, 7);
    }

    #[test]
    fn split_fields_with_invalid_content() {
        assert!(split_fields("3").is_err());
    }
}
