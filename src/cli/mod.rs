use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(version)]
#[command(about = "A simple dotfiles manager that doesn't pollute the system", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: CliCmd,

    /// Auto-answer yes to all prompts
    #[arg(short = 'y', long, global = true, conflicts_with = "assume_no")]
    assume_yes: bool,

    /// Auto-answer no to all prompts
    #[arg(short = 'n', long, global = true, conflicts_with = "assume_yes")]
    assume_no: bool,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum CliCmd {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        dbg!(self);
        Ok(())
    }
}
