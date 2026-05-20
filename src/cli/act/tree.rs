use crate::{
    cli::{Cli, CliCmd, ctx::CliContext},
    prof::TraverseOpts,
};

impl Cli {
    pub fn action_tree(&self) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Tree { no_dedup } => {
                let ctx = CliContext::new(&self.home, &self.root)?;
                let trav_opts = TraverseOpts::new(no_dedup);
                ctx.profiles().traverse(&self.profile, trav_opts, |ctx| {
                    println!("{:?}", ctx.item);
                    Ok(())
                })
            }
            _ => unreachable!("Tree command should be tree"),
        }
    }
}
