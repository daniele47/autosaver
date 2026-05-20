use crate::cli::{Cli, CliCmd, ctx::CliContext};

impl Cli {
    pub fn action_tree(&self, ctx: &CliContext) -> anyhow::Result<()> {
        let _ = ctx;
        match self.cmd {
            CliCmd::Tree { no_dedup } => {}
            _ => unreachable!("Tree command should be tree"),
        }
        todo!()
    }
}
