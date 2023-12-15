mod args;
mod cli;
mod gui;
mod source;

use std::process::{ExitCode, Termination};

#[derive(Debug)]
enum MainResult {
    Result(anyhow::Result<()>),
    ExitCode(gtk::glib::ExitCode),
}
impl Termination for MainResult {
    fn report(self) -> ExitCode {
        match self {
            MainResult::Result(r) => r.report(),
            MainResult::ExitCode(e) => e.report(),
        }
    }
}

fn main() -> MainResult {
    if let Err(e) = ffmpeg::init() {
        return MainResult::Result(Err(e.into()));
    }
    if std::env::args().nth(1).is_none() {
        MainResult::ExitCode(gui::main())
    } else {
        MainResult::Result(cli::main())
    }
}
