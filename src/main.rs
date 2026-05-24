use std::process::exit;

use anyhow::Result;
use autosaver::{cli::Cli, errnow, error, outnow};
use clap::Parser;

fn main() -> Result<()> {
    // parse cmdline
    let cli = Cli::parse();

    // run application
    let run_res = cli.run_cmd();

    // assure output streams are flushed
    outnow!();
    errnow!();

    // handle error
    if let Err(err) = run_res {
        error!("{err}");
        exit(1);
    }

    Ok(())
}
