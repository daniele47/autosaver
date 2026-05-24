use std::collections::HashMap;

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
    },
    prof::{ProfileKind, TraverseOpts},
};

impl Cli {
    pub fn action_run(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run => {
                let run_dir = &ctx.paths[&Paths::Run];
                let mut all = HashMap::new();
                let trav_opts = TraverseOpts::default();
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    match ctx.item.kind() {
                        ProfileKind::Runner(runner) => {
                            let run_dir = run_dir.join(ctx.item.id_or(ctx.name))?;
                            runner.resolve(&run_dir, &mut all, |path, policy| {
                                // TODO: run logic!
                                let _ = (path, policy);
                                Ok(())
                            })?;
                        }
                        _ => {}
                    }
                    Ok(())
                })
            }
            _ => unreachable!("Tree command should be tree"),
        }
    }
}
