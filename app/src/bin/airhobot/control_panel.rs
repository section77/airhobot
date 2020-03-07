use airhobot::prelude::*;
use std::sync::{Arc, RwLock};

/// quick and dirty macro to bind the config values to the gui sliders
macro_rules! bind_values {
    ($cfg:expr, $(($struct:ident, $field:ident)),+) => {
        let cfg = Arc::clone(&$cfg);
        std::thread::spawn(move || {
            loop {
                crossbeam_channel::select! {
                    $(
                        recv($field) -> value => {
                            let mut cfg = cfg.write().unwrap();
                            log::debug!("update Config.{} from {} to {}", stringify!($field),
                                        cfg.$struct.$field, value.unwrap());
                            (*cfg).$struct.$field = value.unwrap()
                        },
                    )+
                }
            }
        })
    }
}

pub struct ControlPanel(Option<cv::GUI>, Arc<RwLock<Config>>);
impl ControlPanel {
    pub fn new(cfg: Config) -> (Self, Arc<RwLock<Config>>) {
        let cfg = Arc::new(RwLock::new(cfg));
        (Self(None, Arc::clone(&cfg)), cfg)
    }

    pub fn toggle(&mut self) {
        if self.is_open() {
            self.destroy()
        } else {
            self.show()
        }
    }
    pub fn is_open(&self) -> bool {
        self.0.is_some()
    }

    pub fn destroy(&mut self) {
        if let Some(gui) = self.0.take() {
            gui.destroy();
        }
    }

    pub fn show(&mut self) {
        if self.0.is_some() {
            return;
        }
        let gui = cv::GUI::new("AirHoBot - Controls");

        // create the sliders
        let (blur, erode, dilate, h_offset, s_offset, v_offset) = {
            let cfg = self.1.read().unwrap();
            (
                gui.slider("Blur", cfg.filter.blur, 0..20),
                gui.slider("Erode", cfg.filter.erode, 0..20),
                gui.slider("Dilate", cfg.filter.dilate, 0..20),
                gui.slider("H Offset", cfg.puck.h_offset, 0..150),
                gui.slider("S Offset", cfg.puck.s_offset, 0..150),
                gui.slider("V Offset", cfg.puck.v_offset, 0..150),
            )
        };

        // bind the sliders to the config values
        bind_values!(
            self.1,
            (filter, blur),
            (filter, erode),
            (filter, dilate),
            (pusher, h_offset),
            (pusher, s_offset),
            (pusher, v_offset),
            (puck, h_offset),
            (puck, s_offset),
            (puck, v_offset)
        );

        self.0 = Some(gui);
    }

    pub fn repaint(&mut self) -> Result<()> {
        if let Some(gui) = &self.0 {
            let cfg = self.1.read().unwrap();
            let mut frame = cv::Mat::<cv::BGR>::new(350, 600, &cv::CVType::CV8SC3, cv::RGB::white())?;

            frame.draw_text(
                &format!("Pusher color: {}", cfg.pusher.color),
                &cv::Point::new(10, 20),
                0.6,
                cv::RGB::black(),
                1,
            );

            frame.draw_text(
                &format!("Pusher color range: {}", cfg.pusher.color_range()?),
                &cv::Point::new(10, 50),
                0.6,
                cv::RGB::black(),
                1,
            );

            frame.draw_text(
                &format!("Puck color: {}", cfg.puck.color),
                &cv::Point::new(10, 80),
                0.6,
                cv::RGB::black(),
                1,
            );

            frame.draw_text(
                &format!("Puck color range: {}", cfg.puck.color_range()?),
                &cv::Point::new(10, 110),
                0.6,
                cv::RGB::black(),
                1,
            );

            frame.draw_line(&cv::Point::new(0, 120), &cv::Point::new(1000, 120), cv::RGB::black(), 2);

            let keys = vec![
                "1: select field",
                "2: pick pusher color",
                "3: pick puck color",
                "4: simulate puck (place two points in the field)",
                "5: move pusher",
                "c: show controls",
                "s: save state",
                "q: quit",
                "use the spacebar for pause",
            ];
            let y_start = 140;
            for (n, t) in keys.iter().enumerate() {
                frame.draw_text(
                    t,
                    &cv::Point::new(10, (y_start + n * 25) as i32),
                    0.6,
                    cv::RGB::black(),
                    1,
                );
            }

            gui.show(&frame).unwrap();
        }
        Ok(())
    }
}
