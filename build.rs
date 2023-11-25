use clap::CommandFactory;
use clap_complete::shells;

use std::env;

include!("src/args.rs");

fn main() -> anyhow::Result<()> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Args::command();
    let path = clap_complete::generate_to(
        shells::Bash,
        &mut cmd,
        "silence-with-sound",
        outdir,
    )?;

    println!("cargo:warning=completion file is generated: {path:?}");

    Ok(())
}
