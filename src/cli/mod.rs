use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::fs::rel::RelPathStr;

pub mod actions;
pub mod config;
pub mod inout;
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
    #[arg(short = 'p', long, env = "AUTOSAVER_PROFILE", num_args=1..)]
    profile: Vec<RelPathStr>,

    /// Specify a different home directory to use
    #[arg(short = 'H', long, env = "AUTOSAVER_HOME")]
    home: Option<PathBuf>,

    /// Specify a different root directory to use
    #[arg(short = 'R', long, env = "AUTOSAVER_ROOT")]
    root: Option<PathBuf>,

    /// Get prompted for each profile if to execute it or not
    #[arg(short = 'c', long)]
    choice: bool,

    /// Disable all colored output
    #[arg(short = 'C', long)]
    no_color: bool,

    /// Auto answer to all prompts with the specified answers
    #[arg(short = 'a', long, help_heading = "Prompt Options")]
    auto_answers: Option<String>,

    /// Skip all prompts and checks entirely and list files
    #[arg(short = 'l', long, conflicts_with_all = ["assume_no", "assume_yes"], help_heading = "Prompt Options")]
    list: bool,

    /// Auto-answer yes to all prompts
    #[arg(short = 'y', long, conflicts_with_all = ["assume_no", "list"], help_heading = "Prompt Options")]
    assume_yes: bool,

    /// Auto-answer no to all prompts
    #[arg(short = 'n', long, conflicts_with_all = ["list", "assume_yes"], help_heading = "Prompt Options")]
    assume_no: bool,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum CliCmd {
    /// List changes between home and backup directories
    List {
        #[command(flatten)]
        act_backup: CliActBackup,
    },
    /// Save changes in home directory to the backup
    Save {
        #[command(flatten)]
        act_backup: CliActBackup,

        #[command(flatten)]
        act_saverestore: CliActSaveRestore,

        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,
    },
    /// Restore changes in backup directory to the home
    Restore {
        #[command(flatten)]
        act_backup: CliActBackup,

        #[command(flatten)]
        act_saverestore: CliActSaveRestore,

        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,
    },
    /// Delete tracked dotfiles
    Delete {
        /// Show only files only from home directory
        #[arg(short = 'o', long, conflicts_with = "only_backup")]
        only_original: bool,

        /// Show only files from the backup directory
        #[arg(short = 'b', long, conflicts_with = "only_original")]
        only_backup: bool,

        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,
    },
    /// Run init scripts
    Run {
        /// Enable stdin in scripts that hint their need for it
        #[arg(short = 'i', long)]
        stdin: bool,
    },
    /// Show dependency tree of profiles
    Tree {
        /// Do no deduplicate profiles
        #[arg(short = 'd', long)]
        no_dedup: bool,

        /// Show the id related to each profile
        #[arg(short = 's', long)]
        show_id: bool,

        /// Ignore one or more profiles, if repeated
        #[arg(short = 'i', long, value_name = "PROFILE")]
        ignore: Vec<RelPathStr>,
    },
    /// Clear untracked files in backup directories
    Clear {
        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,
    },
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliActDelSymlinks {
    /// Allow deleting symlink files
    #[arg(short = 's', long, global = true)]
    symlink: bool,
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliActBackup {
    /// Include also paths with notdiff policy
    #[arg(short = 'a', long)]
    all: bool,

    /// Include also paths that do not differ
    #[arg(short = 'u', long)]
    unmodified: bool,
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliActSaveRestore {
    /// Allow deleting files in backup directory
    #[arg(short = 'f', long)]
    force: bool,

    /// Allow duplicated paths, and just warn about them
    #[arg(short = 'd', long, global = true)]
    allow_duplicates: bool,
}
