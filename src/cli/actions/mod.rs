use crate::cli::{Cli, CliCmd, ctx::CliContext};

pub mod backup;
pub mod run;
pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let ctx = CliContext::new(&self.home, &self.root, &self.profile)?;

        match self.cmd {
            CliCmd::List { .. }
            | CliCmd::Save { .. }
            | CliCmd::Restore { .. }
            | CliCmd::Delete { .. } => self.action_backup(&ctx),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear => todo!(),
        }?;

        Ok(())
    }
}
