use crate::cli::{Cli, CliCmd, ctx::CliContext};

impl Cli {
    pub fn action_tree(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Tree { no_dedup } => {
                let _ = (no_dedup, ctx);
                todo!("IMPLEMENT TREE COMMAND!");
            }
            _ => unreachable!("Tree command should be tree"),
        }
    }
}
