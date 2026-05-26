use crate::cli::{Cli, CliCmd, ctx::CliContext};

pub mod run;
pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let ctx = CliContext::new(&self.home, &self.root, &self.profile)?;

        match self.cmd {
            CliCmd::List => todo!(),
            CliCmd::Save => todo!(),
            CliCmd::Restore => todo!(),
            CliCmd::Delete => todo!(),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear => todo!(),
        }?;

        Ok(())
    }
}
