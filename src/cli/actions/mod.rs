use crate::cli::{Cli, CliCmd, config::CliContext, prompt::Prompt};

pub mod backup;
pub mod clear;
pub mod run;
pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let prompt = Prompt::new(
            self.auto_answers.clone().unwrap_or_default(),
            self.assume_yes,
            self.assume_no,
            self.list,
        )?;
        let ctx = CliContext::new(
            &self.home,
            &self.root,
            &self.profile,
            &self.profiles,
            prompt,
        )?;

        match self.cmd {
            CliCmd::List { .. }
            | CliCmd::Save { .. }
            | CliCmd::Restore { .. }
            | CliCmd::Delete { .. } => self.action_backup(&ctx),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear => self.action_clear(&ctx),
        }
    }
}
