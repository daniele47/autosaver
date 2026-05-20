use crate::cli::Cli;

pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        match self.cmd {
            super::CliCmd::List => todo!(),
            super::CliCmd::Save => todo!(),
            super::CliCmd::Restore => todo!(),
            super::CliCmd::Delete => todo!(),
            super::CliCmd::Run => todo!(),
            super::CliCmd::Tree { .. } => self.action_tree(),
            super::CliCmd::Clear => todo!(),
        }
    }
}
