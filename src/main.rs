use anyhow::Result;
use autosaver::cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    // parse cmdline
    let cli = Cli::parse();

    // run application
    cli.run()?;

    Ok(())
}
