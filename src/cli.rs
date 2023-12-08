use crate::args::Args;
use crate::source::AudioConfig;

use clap::Parser;
use rand::distributions::Uniform;
use rand::Rng;
use rodio::{OutputStream, Sink, Source};

use std::time::Duration;

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let is_test_run = args.test_run;

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let mut rng = rand::thread_rng();
    let range =
        Uniform::new(Duration::from_secs(0), Duration::from_secs(60 * 5));

    let source = AudioConfig::try_from(args)?.generate()?.buffered();

    if is_test_run {
        sink.append(source);
        sink.sleep_until_end();
        return Ok(());
    }

    loop {
        let wait = rng.sample(range);
        std::thread::sleep(wait);

        sink.append(source.clone());
        sink.sleep_until_end();
    }
}
