use crate::cli::{Cli, CliCmd, config::CliContext, perf};

pub mod backup;
pub mod clear;
pub mod run;
pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let ctx = perf("    - Configs parsed -->", || {
            CliContext::new(&self.home, &self.root, &self.profile, &self.profiles)
        })?;

        perf("    - Command run    -->", || match self.cmd {
            CliCmd::List { .. }
            | CliCmd::Save { .. }
            | CliCmd::Restore { .. }
            | CliCmd::Delete { .. } => self.action_backup(&ctx),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear => self.action_clear(&ctx),
        })?;

        Ok(())
    }
}
