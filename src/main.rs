use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

pub mod cli;
pub mod fs;
pub mod prof;

fn main() -> Result<()> {
    Cli::parse().run()
}
