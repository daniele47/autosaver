use crate::cli::{Cli, CliCmd, config::CliContext, prompt::Prompt};

pub mod backup;
pub mod clear;
pub mod run;
pub mod tree;

impl Cli {
    const SYMLINK_FLAG: &str = "--allow_symlink";
    const DELETE_FLAG: &str = "--allow_purge";

    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let prompt = Prompt::new(
            self.auto_answers.as_deref().unwrap_or(""),
            self.assume_yes,
            self.assume_no,
            self.dry_run,
        )?;
        let ctx = CliContext::new(&self.home, &self.root, &self.profile, prompt)?;

        match self.cmd {
            CliCmd::List { .. }
            | CliCmd::Save { .. }
            | CliCmd::Restore { .. }
            | CliCmd::Delete { .. } => self.action_backup(&ctx),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear { .. } => self.action_clear(&ctx),
        }
    }
}
