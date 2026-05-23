use anyhow::Result;
use autosaver::{cli::Cli, error};
use clap::Parser;

fn main() -> Result<()> {
    // parse cmdline
    let cli = Cli::parse();

    // run application
    if let Err(err) = cli.run_cmd() {
        error!("{err}");
    }

    Ok(())
}
