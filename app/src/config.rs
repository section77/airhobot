use crate::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::{fs, ops::Range, path::PathBuf, net::SocketAddrV4, sync::{Arc, RwLock}};

///
pub type Cfg = Arc<RwLock<Config>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub roi: Roi,
    pub filter: Filter,
    pub pusher: Detector,
    pub puck: Detector,
    pub driver: Driver,
}


impl Config {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        info!("load configuration from {}", path.display());
        let str_value = fs::read_to_string(&path)?;
        let config = serde_json::from_str(&str_value)?;
        Ok(config)
    }

    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        info!("save configuration to {}", path.display());
        let str_value = serde_json::to_string_pretty(self)?;
        fs::write(&path, str_value)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            roi: Roi::new(960, 550),
            filter: Filter::default(),
            pusher: Detector {
                color: cv::HSV::unsafe_new(0, 220, 140),
                h_offset: 30,
                s_offset: 43,
                v_offset: 35,
                area_range: 350.0..3000.0,
                min_vertices: 8,
            },
            puck: Detector {
                color: cv::HSV::unsafe_new(60, 120, 120),
                h_offset: 30,
                s_offset: 40,
                v_offset: 50,
                area_range: 350.0..3000.0,
                min_vertices: 8,
            },
            driver: Driver::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone)]
pub struct Roi {
    pub lt: cv::Point,
    pub rt: cv::Point,
    pub rb: cv::Point,
    pub lb: cv::Point,
}
impl Roi {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            lt: cv::Point::new(0, 0),
            rt: cv::Point::new(w, 0),
            rb: cv::Point::new(w, h),
            lb: cv::Point::new(0, h),
        }
    }

    pub fn from_array(ps: [cv::Point; 4]) -> Self {
        Self {
            lt: ps[0],
            rt: ps[1],
            rb: ps[2],
            lb: ps[3],
        }
    }

    pub fn from_vec(v: Vec<cv::Point>) -> Result<Self> {
        if v.len() == 4 {
            Ok(Self {
                lt: v[0],
                rt: v[1],
                rb: v[2],
                lb: v[3],
            })
        } else {
            Err(format!("Roi::from_vec only valid from vec with 4 elements - given vec: {:?}", v).into())
        }
    }

    pub fn from_frame<T>(frame: &cv::Mat<T>) -> Self {
        let w = frame.n_cols();
        let h = frame.n_rows();
        Self::new(w, h)
    }

    pub fn to_array(&self) -> [cv::Point; 4] {
        [self.lt, self.rt, self.rb, self.lb]
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Filter {
    pub blur: i32,
    pub erode: i32,
    pub dilate: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detector {
    pub color: cv::HSV,
    pub h_offset: i32,
    pub s_offset: i32,
    pub v_offset: i32,
    pub area_range: Range<f64>,
    pub min_vertices: usize,
}

impl Detector {
    pub fn color_range(&self) -> Result<cv::HSVRange> {
        let color_offsets = (self.h_offset, self.s_offset, self.v_offset);
        let color_range = cv::HSVRange::from_hsv(&self.color, color_offsets)?;
        Ok(color_range)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Driver {
    pub addr: SocketAddrV4,
    pub delay: u32,
    pub factor: f32,
}

impl Default for Driver {
    fn default() -> Self {
        Self {
            addr: "192.168.1.100:6789".parse().unwrap(),
            delay: 450,
            factor: 0.005,
        }
    }
}
