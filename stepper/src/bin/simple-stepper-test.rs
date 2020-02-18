use env_logger::{Builder, Env};
use log::info;
use std::{
    sync::{
        Arc,
        Mutex,
    },
    thread,
    time::Duration
};
use stepper::*;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let n_steps = std::env::args().nth(1).map(|x| x.parse().unwrap()).unwrap_or(500);
    let delay = std::env::args().nth(2).map(|x| x.parse().unwrap()).unwrap_or(600);
    println!("n_steps: {}", n_steps);

    let stepper1 = Arc::new(Mutex::new(Stepper::new("1", EnablePin(23), StepPin(27), DirectionPin(22)).unwrap()));
    let stepper2 = Arc::new(Mutex::new(Stepper::new("2", EnablePin(24), StepPin(13), DirectionPin(17)).unwrap()));
    loop {
        let stepper1 = stepper1.clone();
        let hndl = {
            thread::spawn(move || run_stepper(&mut stepper1.lock().unwrap(), n_steps, delay))
        };

        let stepper2 = stepper2.clone();
        run_stepper(&mut stepper2.lock().unwrap(), n_steps, delay);
        hndl.join().unwrap();
    }
}

fn run_stepper(stepper: &mut Stepper, n_steps: u32, delay: i32) {
    info!("{} - {} Schritte nach links", stepper, n_steps);
    stepper.step_n(Direction::Left, n_steps, delay);

    thread::sleep(Duration::from_millis(1000));

    info!("{} - {} Schritte nach rechts", stepper, n_steps);
    stepper.step_n(Direction::Right, n_steps, delay);

    thread::sleep(Duration::from_millis(1000));
}
