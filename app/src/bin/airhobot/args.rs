use airhobot::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

///
/// KEYBOARD SHORTCUTS:
///
///   1: select field
///   2: pick pusher color
///   3: pick puck color
///   4: simulate puck (place two points in the field)
///   5: move pusher
///   c: show controls
///   r: reload config
///   f: next frame
///   s: save state
///   q: quit
///
///   use the spacebar for pause
#[derive(StructOpt, Debug)]
#[structopt(name = "AirHoBot", verbatim_doc_comment)]
pub struct Args {
    /// config file
    #[structopt(short, long, default_value = "airhobot.toml")]
    pub config_file: PathBuf,

    /// verbose logging (use -vv for trace logging)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// show frame for (in millis)
    #[structopt(short, long, default_value = "50")]
    pub delay: u64,

    /// Use the cam with the given id as input source
    #[structopt(long, conflicts_with = "image, video")]
    pub cam: Option<i32>,

    /// Use the image as input source
    #[structopt(long, conflicts_with = "cam_id, video")]
    pub image: Option<PathBuf>,

    /// Use the video as input source
    #[structopt(long, conflicts_with = "image, cam_id")]
    pub video: Option<PathBuf>,
}

impl Args {
    pub fn source(&self) -> Result<Source> {
        self.cam
            .map(Source::cam)
            .or(self.video.as_ref().map(Source::video))
            .or(self.image.as_ref().map(Source::image))
            .unwrap_or(Err(Error::Arguments {
                msg: "input source missing - use the `-h` flag for help".into(),
            }))
    }
}
