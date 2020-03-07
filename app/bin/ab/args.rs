use structopt::StructOpt;
use airhobot::prelude::*;
use std::{
    path::PathBuf,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "AirHoBot")]
pub struct Args {
    /// Use the cam with the given id as input source
    #[structopt(short, long, conflicts_with = "image")]
    pub cam_id: Option<i32>,

    /// Use the image as input source
    #[structopt(short, long, conflicts_with = "cam_id")]
    pub image: Option<PathBuf>,

    /// Use the video as input source
    #[structopt(short, long, conflicts_with = "image, cam_id")]
    pub video: Option<PathBuf>,
}

impl Args {
    pub fn source(&self) -> Result<Source> {
        self.cam_id
            .map(Source::cam)
            .or(self.video.as_ref().map(Source::video))
            .or(self.image.as_ref().map(Source::image))
            .unwrap_or(Err(Error::Arguments {
                msg: "input source missing".into(),
            }))
    }
}
