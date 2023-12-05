mod args;
mod source;

use args::Args;
use source::AudioConfig;

use clap::Parser;
use rand::distributions::Uniform;
use rand::Rng;
use rodio::{OutputStream, Sink, Source};

use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.set_volume(args.volume);

    let mut rng = rand::thread_rng();
    let range =
        Uniform::new(Duration::from_secs(0), Duration::from_secs(60 * 5));

    let source = AudioConfig::try_from(args)?.generate()?.buffered();
    loop {
        let wait = rng.sample(range);
        std::thread::sleep(wait);

        sink.append(source.clone());
        sink.sleep_until_end();
    }
}
