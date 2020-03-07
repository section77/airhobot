use crate::prelude::*;
use std::{
    time::Instant,
    collections::VecDeque,
};

#[derive(Debug)]
pub struct State {
    pub ts: Instant,
    pub cfg: Cfg,
    pub frame: cv::Mat<cv::BGR>,
    pub frame_orig: cv::Mat<cv::BGR>,
    pub pusher: Option<cv::Point>,
    pub puck: Option<cv::Point>,
    pub history: VecDeque<State>,
}

impl State {
    pub fn new(cfg: Cfg, frame: cv::Mat<cv::BGR>) -> Self {
        Self {
            ts: Instant::now(),
            cfg,
            frame: frame.clone(),
            frame_orig: frame,
            pusher: None,
            puck: None,
            history: Default::default(),
        }
    }

    pub fn next(mut self, frame: cv::Mat<cv::BGR>) -> Self {

        let cfg = self.cfg.clone();
        let mut history = std::mem::take(&mut self.history);
        history.push_front(self);
        history.truncate(10);

        Self {
            ts: Instant::now(),
            cfg,
            frame: frame.clone(),
            frame_orig: frame,
            pusher: None,
            puck: None,
            history: Default::default(),
        }
    }

    pub fn crop_frame(&mut self) -> Result<()> {
        let roi = self.cfg.read()?.roi;
        self.frame = self.frame.lens(&roi.to_array())?;
        Ok(())
    }

    pub fn apply_filter_frame(&mut self) -> Result<()> {
        let filter = &self.cfg.read()?.filter;
        self.frame.blur(filter.blur);
        self.frame.erode(filter.erode);
        self.frame.dilate(filter.dilate);
        Ok(())
    }


    pub fn puck_speed(&self) -> Option<PuckSpeed> {
        let puck = self.puck?;
        let old_state = self.history.iter().find(|s| s.puck.is_some())?;
        let old_puck = old_state.puck?;

        let dur = self.ts.duration_since(old_state.ts);
        let dist = puck.dist(&old_puck);
        Some(PuckSpeed::new(dur, dist))
    }

    // pub fn draw_roi<T>(&self, frame: &mut cv::Mat<T>) {
    //     let roi = &self.cfg.roi;
    //     for (i, from) in roi.iter().enumerate() {
    //         let to = roi.get(i + 1).unwrap_or(&roi[0]);
    //         frame.draw_line(from, to, cv::RGB::red(), 2);
    //     }
    // }
}

