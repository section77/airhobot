use log::{debug, info};
use rppal::gpio;
use std::error::Error;
use std::fmt;
use std::{thread::sleep, time::Duration};

/// Direction
#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

/// GPIO pin number to the 'Enable' pin on the stepper-driver
#[derive(Debug, PartialEq)]
pub struct EnablePin(pub u8);

/// GPIO pin number to the 'Step' pin on the stepper-driver
#[derive(Debug, PartialEq)]
pub struct StepPin(pub u8);

/// GPIO pin number to the 'Dir' pin on the stepper-driver
#[derive(Debug, PartialEq)]
pub struct DirectionPin(pub u8);

/// Represents a stepper
#[derive(Debug)]
pub struct Stepper {
    name: String,
    pin_enable: gpio::OutputPin,
    pin_step: gpio::OutputPin,
    pin_direction: gpio::OutputPin,
    current_direction: Direction,
}

impl fmt::Display for Stepper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Stepper: {} - pins - enable: {:?}, step: {:?}, direction: {:?}",
            self.name,
            self.pin_enable.pin(),
            self.pin_step.pin(),
            self.pin_direction.pin()
        )
    }
}

impl Stepper {
    /// Initialize the stepper
    pub fn new(name: &str, e: EnablePin, s: StepPin, d: DirectionPin) -> Result<Stepper, Box<dyn Error>> {
        let pin_enable = gpio::Gpio::new()?.get(e.0)?.into_output();
        let pin_step = gpio::Gpio::new()?.get(s.0)?.into_output();
        let pin_direction = gpio::Gpio::new()?.get(d.0)?.into_output();

        // FIXME: enable stepper here?

        let stepper = Stepper {
            name: name.into(),
            pin_enable,
            pin_step,
            pin_direction,
            current_direction: Direction::Left,
        };

        info!("new {}", stepper);
        Ok(stepper)
    }

    pub fn enable(&mut self) {
        debug!("{} - enable", self.name);
        self.pin_enable.set_high();
    }

    pub fn disable(&mut self) {
        debug!("{} - disable", self.name);
        self.pin_enable.set_low();
    }

    pub fn step(&mut self, direction: Direction) {
        debug!("{} - step - direction: {:?}", self.name, direction);
        self.set_direction(direction);
        self.pin_step.set_high();
        sleep(Duration::from_micros(600));
        self.pin_step.set_low();
        sleep(Duration::from_micros(600));
    }

    pub fn step_n(&mut self, direction: Direction, steps: u32, delay: i32) {
        debug!(
            "{} - step_n - direction: {:?}, steps: {}, delay: {}",
            self.name, direction, steps, delay
        );
        self.set_direction(direction);

        for _ in 0..steps {
            //let delay = 600; // max(400, n - i as i32 * 2) as u64;
            self.pin_step.set_high();
            sleep(Duration::from_micros(delay as u64));
            self.pin_step.set_low();
            sleep(Duration::from_micros(delay as u64));
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Right => {
                if self.current_direction == Left {
                    debug!("{} - switch direction to {:?}", self.name, Right);
                    self.pin_direction.set_high();
                    self.current_direction = Right;
                }
            }
            Left => {
                if self.current_direction == Right {
                    debug!("{} - switch direction to {:?}", self.name, Left);
                    self.pin_direction.set_low();
                    self.current_direction = Left;
                }
            }
        }
    }
}
