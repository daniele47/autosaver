use anyhow::Result;

use crate::cli::Cli;

pub mod cli;
pub mod fs;
pub mod prof;

fn main() -> Result<()> {
    Cli::run()
}
