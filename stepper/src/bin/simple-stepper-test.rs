use env_logger::{Builder, Env};
use log::info;
use std::{thread, time::Duration};
use stepper::*;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut stepper1 = Stepper::new("1", EnablePin(23), StepPin(27), DirectionPin(17)).unwrap();
    let mut stepper2 = Stepper::new("2", EnablePin(24), StepPin(13), DirectionPin(5)).unwrap();

    loop {
        let n_steps = 500;
        run_stepper(&mut stepper1, n_steps);
        run_stepper(&mut stepper2, n_steps);
    }
}

fn run_stepper(stepper: &mut Stepper, n_steps: u32) {
    info!("{} - {} Schritte nach links", stepper, n_steps);
    stepper.step_n(Direction::Left, n_steps);

    thread::sleep(Duration::from_millis(1000));

    info!("{} - {} Schritte nach rechts", stepper, n_steps);
    stepper.step_n(Direction::Right, n_steps);

    thread::sleep(Duration::from_millis(1000));
}
