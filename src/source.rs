use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use derive_builder::Builder;
use rodio::{Decoder, Source};

pub type BoxSource<T> = Box<dyn Source<Item = T> + Send + 'static>;

#[derive(Debug, Builder)]
pub struct AudioConfig {
    file_name: Box<str>,

    #[builder(default = "Duration::ZERO")]
    start: Duration,
    #[builder(default = "Duration::MAX")]
    duration: Duration,

    #[builder(default = "1.0")]
    volume: f32,
}

impl AudioConfig {
    pub fn generate(&self) -> anyhow::Result<BoxSource<i16>> {
        let file = File::open(&*self.file_name)?;
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
