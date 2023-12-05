mod args;
mod source;

use args::Args;

use clap::Parser;
use rand::distributions::Uniform;
use rand::Rng;
use rodio::{OutputStream, Sink, Source};

use crate::source::{AudioConfig, AudioConfigBuilder};
use std::time::Duration;

fn generate_config(args: Args) -> anyhow::Result<AudioConfig> {
    let mut builder = AudioConfigBuilder::default();
    builder.file_name(args.audio_file).volume(args.volume);

    args.audio_start.map(|s| builder.start(s));
    args.audio_duration.map(|d| builder.duration(d));
    args.audio_end.map(|e| {
        let s = args.audio_start.unwrap_or(Duration::ZERO);
        let d = e - s;
        builder.duration(d);
    });

    builder.build().map_err(|e| e.into())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.set_volume(args.volume);

    let mut rng = rand::thread_rng();
    let range =
        Uniform::new(Duration::from_secs(0), Duration::from_secs(60 * 5));

    let source = generate_config(args)?.generate()?.buffered();
    loop {
        let wait = rng.sample(range);
        std::thread::sleep(wait);

        sink.append(source.clone());
        sink.sleep_until_end();
    }
}
