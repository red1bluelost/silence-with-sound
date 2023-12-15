use crate::args::Args;

use derive_builder::Builder;
use rodio::{Decoder, Source};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

pub type BoxSource<T> = Box<dyn Source<Item = T> + Send + 'static>;

#[derive(Debug, Builder)]
pub struct AudioConfig {
    file_name: Box<Path>,

    #[builder(default = "Duration::ZERO")]
    start: Duration,
    #[builder(default = "Duration::MAX")]
    duration: Duration,

    #[builder(default = "1.0")]
    volume: f32,
}

impl TryFrom<Args> for AudioConfig {
    type Error = AudioConfigBuilderError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut builder = AudioConfigBuilder::default();
        builder.file_name(args.audio_file).volume(args.volume);

        if let Some(s) = args.audio_start {
            builder.start(s);
        }
        if let Some(d) = args.audio_duration {
            builder.duration(d);
        }
        if let Some(e) = args.audio_end {
            let s = args.audio_start.unwrap_or(Duration::ZERO);
            let d = e - s;
            builder.duration(d);
        }

        builder.build()
    }
}

impl AudioConfig {
    pub fn generate(&self) -> anyhow::Result<BoxSource<i16>> {
        let file = File::open(&self.file_name)?;
        let file = BufReader::new(file);
        let mut source: BoxSource<_> = Box::new(Decoder::new(file)?);
        if self.start != Duration::ZERO {
            source = Box::new(source.skip_duration(self.start));
        }
        if self.duration != Duration::MAX {
            source = Box::new(source.take_duration(self.duration));
        }
        if self.volume != 1.0 {
            source = Box::new(source.amplify(self.volume));
        }
        Ok(source)
    }
}
