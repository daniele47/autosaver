use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {}

impl Cli {
    pub fn run() -> Result<()> {
        let _ = Cli::parse();
        Ok(())
    }
}
