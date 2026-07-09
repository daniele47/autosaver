use std::process::exit;

use autosaver::{cli::Cli, error};
use clap::Parser;

fn main() {
    // parse cmdline
    let cli = Cli::parse();
    // run application
    let run_res = cli.run_cmd();

    // handle error
    if let Err(err) = run_res {
        error!("{err:?}");
        exit(1);
    }
}
