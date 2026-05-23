use crate::{
    cli::{Cli, CliCmd, ctx::CliContext},
    prof::TraverseOpts,
};

impl Cli {
    pub fn action_run(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run {} => {
                let trav_opts = TraverseOpts::default();
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    let i = {};
                    Ok(())
                })
            }
            _ => unreachable!("Tree command should be tree"),
        }
    }
}
