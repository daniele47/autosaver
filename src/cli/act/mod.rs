use crate::cli::{Cli, ctx::CliContext};

pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let ctx = CliContext::new(&self.home, &self.root)?;

        match self.cmd {
            super::CliCmd::List => todo!(),
            super::CliCmd::Save => todo!(),
            super::CliCmd::Restore => todo!(),
            super::CliCmd::Delete => todo!(),
            super::CliCmd::Run => todo!(),
            super::CliCmd::Tree { .. } => self.action_tree(&ctx),
            super::CliCmd::Clear => todo!(),
        }
    }
}
