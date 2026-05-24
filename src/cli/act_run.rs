use std::collections::HashMap;

use owo_colors::OwoColorize;

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
        prompt::{Prompt, PromptFlags},
    },
    outln,
    prof::{ProfileKind, TraverseOpts, runner::RunnerPolicy},
};

impl Cli {
    pub fn action_run(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run => {
                let run_dir = &ctx.paths[&Paths::Run];
                let mut all = HashMap::new();
                let trav_opts = TraverseOpts::default();
                let mut prompt = Prompt::new(PromptFlags::all() & !PromptFlags::DIFF);

                // traverse all runner profiles
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    if let ProfileKind::Runner(runner) = ctx.item.kind() {
                        let this_run_dir = run_dir.join(ctx.item.id_or(ctx.name))?;
                        runner.resolve(&this_run_dir, &mut all, |path, policy| {
                            // filter entries with skip policy
                            if policy == RunnerPolicy::Skip {
                                return Ok(());
                            }

                            // prompt user
                            let relpath = path.to_rel(run_dir)?;
                            outln!("{}", relpath.display().style(CliContext::OUTPUT_PATH));
                            let msg = "Do you really want to run the script?";
                            let paths = &[&path];
                            prompt.handled_prompt(msg, paths, || {
                                let _ = "TODO";
                                Ok(())
                            })?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })
            }
            _ => unreachable!("Tree command should be tree"),
        }
    }
}
