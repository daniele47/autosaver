use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::fs::rel::RelPathStr;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(version)]
#[command(infer_subcommands = true)]
#[command(disable_help_subcommand = true)]
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

    /// Specify which profile to use
    #[arg(short, long, env = "AUTOSAVER_PROFILE")]
    profile: Option<RelPathStr>,

    /// Specify a different home directory to use
    #[arg(long, env = "AUTOSAVER_HOME")]
    home: Option<RelPathStr>,

    /// Specify a different root directory to use
    #[arg(long, env = "AUTOSAVER_ROOT")]
    root: Option<RelPathStr>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum CliCmd {
    /// List changes between home and backup directories
    List,
    /// Save changes in home directory to the backup
    Save,
    /// Restore changes in backup directory to the home
    Restore,
    /// Delete tracked dotfiles
    Delete,
    /// Run init scripts
    Run,
    /// Show dependency tree of profiles
    Tree,
    /// Clear untracked files in backup directories
    Clear,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        dbg!(self);
        Ok(())
    }
}
