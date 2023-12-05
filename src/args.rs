use clap::{ArgGroup, Parser};

use std::time::Duration;

#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("audio length")
        .required(false)
        .args(&["audio_end", "audio_duration"]),
))]
pub struct Args {
    pub audio_file: Box<str>,

    #[arg(long, default_value_t = 1.0)]
    pub volume: f32,

    #[arg(long, value_parser = humantime::parse_duration)]
    pub audio_start: Option<Duration>,
    #[arg(long, value_parser = humantime::parse_duration)]
    pub audio_end: Option<Duration>,
    #[arg(long, value_parser = humantime::parse_duration)]
    pub audio_duration: Option<Duration>,
}
