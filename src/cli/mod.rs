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
    pub cmd: CliCmd,

    /// Specify which profile to use
    #[arg(short = 'p', long, env = "AUTOSAVER_PROFILE")]
    pub profile: Option<RelPathStr>,

    /// Specify profiles to exclude
    #[arg(short = 'e', long, value_name = "PROFILE")]
    pub exclude_profile: Vec<RelPathStr>,

    /// Specify a different home directory to use
    #[arg(short = 'H', long, env = "AUTOSAVER_HOME")]
    pub home: Option<PathBuf>,

    /// Specify a different root directory to use
    #[arg(short = 'R', long, env = "AUTOSAVER_ROOT")]
    pub root: Option<PathBuf>,

    /// Get prompted for each profile before executing it
    #[arg(short = 'c', long)]
    pub choice: bool,

    /// Auto answer to all prompts with the specified answers
    #[arg(short = 'a', long, help_heading = "Prompt Options")]
    pub auto_answers: Option<String>,

    /// Auto-answer yes to all prompts
    #[arg(short = 'y', long, conflicts_with_all = ["assume_no", "dry_run"], help_heading = "Prompt Options")]
    pub assume_yes: bool,

    /// Auto-answer no to all prompts
    #[arg(short = 'n', long, conflicts_with_all = ["dry_run", "assume_yes"], help_heading = "Prompt Options")]
    pub assume_no: bool,

    /// Skip all prompts and checks entirely
    #[arg(short = 'd', long, conflicts_with_all = ["assume_no", "assume_yes"], help_heading = "Prompt Options")]
    pub dry_run: bool,
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
        act_saverestore: CliActSaveRestore,

        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,

        #[command(flatten)]
        act_backup: CliActBackup,
    },
    /// Restore changes in backup directory to the home
    Restore {
        #[command(flatten)]
        act_saverestore: CliActSaveRestore,

        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,

        /// Delete also cleanup paths specified in config files
        #[arg(short = 'c', long)]
        allow_cleanup: bool,

        #[command(flatten)]
        act_backup: CliActBackup,
    },
    /// Delete tracked dotfiles
    Delete {
        /// Delete only cleanup paths specified by profile configs
        #[arg(short = 'c', long)]
        only_cleanup: bool,

        /// Delete only files from the backup directory
        #[arg(short = 'b', long)]
        only_backup: bool,

        /// Delete only files from the original home directory
        #[arg(short = 'o', long)]
        only_original: bool,

        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,
    },
    /// Run init scripts
    Run {
        /// Enable stdin in scripts that hint their need for it
        #[arg(short = 's', long)]
        allow_stdin: bool,
    },
    /// Show dependency tree of profiles
    Tree {
        /// Show duplicated profiles that appear multiple times
        #[arg(short = 'd', long)]
        show_dups: bool,

        /// Show the profile id in the tree visualization
        #[arg(short = 'i', long)]
        show_id: bool,
    },
    /// Clear untracked files in backup directory
    Clear {
        #[command(flatten)]
        act_delsymlinks: CliActDelSymlinks,
    },
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliActSaveRestore {
    /// Allow duplicated paths, and just warn about them
    #[arg(short = 'd', long, global = true)]
    pub allow_duplicates: bool,

    /// Allow deleting files in backup directory
    #[arg(short = 'p', long)]
    pub allow_purge: bool,
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliActDelSymlinks {
    /// Allow deleting symlink files
    #[arg(short = 's', long, global = true)]
    pub allow_symlink: bool,
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliActBackup {
    /// Show also paths with notdiff policy
    #[arg(short = 'e', long)]
    pub show_excluded: bool,

    /// Show also paths that do not differ
    #[arg(short = 'u', long)]
    pub show_unmodified: bool,
}
