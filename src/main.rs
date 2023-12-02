mod args;

use args::Args;

use clap::Parser;
use rand::distributions::Uniform;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink, Source};

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

type BoxSource<T> = Box<dyn Source<Item = T> + Send + 'static>;

fn generate_source(args: &Args) -> anyhow::Result<BoxSource<i16>> {
    let audio_file = File::open(&args.audio_file)?;
    let file = BufReader::new(audio_file);
    let mut source: BoxSource<_> = Box::new(Decoder::new(file)?);

    if let Some(audio_start) = args.audio_start {
        source = Box::new(source.skip_duration(audio_start));
    }

    if let Some(audio_end) = args.audio_end {
        let audio_start = args.audio_start.unwrap_or(Duration::from_secs(0));
        let audio_duration = audio_end - audio_start;
        source = Box::new(source.take_duration(audio_duration));
    }

    if let Some(audio_duration) = args.audio_duration {
        source = Box::new(source.take_duration(audio_duration));
    }
    Ok(source)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.set_volume(args.volume);

    let mut rng = rand::thread_rng();
    let range =
        Uniform::new(Duration::from_secs(0), Duration::from_secs(60 * 5));

    let source = generate_source(&args)?.buffered();
    loop {
        let wait = rng.sample(range);
        std::thread::sleep(wait);

        sink.append(source.clone());
        sink.sleep_until_end();
    }
}
