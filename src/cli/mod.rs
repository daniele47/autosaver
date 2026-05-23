use std::{path::PathBuf, time::Instant};

use clap::{Parser, Subcommand};

use crate::{
    cli::ctx::{CliContext, Paths},
    fs::rel::RelPathStr,
    verbose,
};

pub mod act_tree;
pub mod ctx;
pub mod out;
pub mod prompt;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(version)]
#[command(infer_subcommands = true)]
#[command(disable_help_subcommand = true)]
#[command(about = "A simple dotfiles manager that doesn't pollute the system", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: CliCmd,

    /// Specify which profile to use
    #[arg(short, long, env = "AUTOSAVER_PROFILE")]
    profile: Option<RelPathStr>,

    /// Specify a different home directory to use
    #[arg(long, env = "AUTOSAVER_HOME")]
    home: Option<PathBuf>,

    /// Specify a different root directory to use
    #[arg(long, env = "AUTOSAVER_ROOT")]
    root: Option<PathBuf>,

    /// Show verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Auto-answer yes to all prompts
    #[arg(short = 'y', long, global = true, conflicts_with = "assume_no")]
    assume_yes: bool,

    /// Auto-answer no to all prompts
    #[arg(short = 'n', long, global = true, conflicts_with = "assume_yes")]
    assume_no: bool,
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
    Tree {
        /// Do no deduplicate profiles
        #[arg(short = 'd', long)]
        no_dedup: bool,

        /// Show the id related to each profile
        #[arg(short = 'i', long)]
        show_id: bool,
    },
    /// Clear untracked files in backup directories
    Clear,
}

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let start = Instant::now();
        let ctx = CliContext::new(&self.home, &self.root, &self.profile)?;

        // verbose output
        if self.verbose {
            verbose!("Configs parsed in {}s", start.elapsed().as_secs_f64());
            verbose!("Current profile: {}", ctx.curr_profile.display());
            verbose!("Home directory: {}", ctx.paths[&Paths::Home].display());
            verbose!("Root directory: {}", ctx.paths[&Paths::Root].display());
            verbose!("Backup directory: {}", ctx.paths[&Paths::Backup].display());
            verbose!("Config directory: {}", ctx.paths[&Paths::Config].display());
            verbose!("Run directory: {}", ctx.paths[&Paths::Run].display());
        }
        let start = Instant::now();

        match self.cmd {
            CliCmd::List => todo!(),
            CliCmd::Save => todo!(),
            CliCmd::Restore => todo!(),
            CliCmd::Delete => todo!(),
            CliCmd::Run => todo!(),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear => todo!(),
        }?;

        // show run time on verbose output
        if self.verbose {
            verbose!("Command run in {}s", start.elapsed().as_secs_f64());
        }

        Ok(())
    }
}
