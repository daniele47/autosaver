use std::process::{Command, Stdio};

use anyhow::{Context, bail};
use owo_colors::OwoColorize;

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
        prompt::{Prompt, PromptAnswer, PromptFlags},
    },
    outln,
    prof::{ProfileKind, TraverseOpts, runner::RunnerPolicy},
};

impl Cli {
    pub fn action_run(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run { interactive } => {
                let run_dir = &ctx.paths[&Paths::Run];
                let trav_opts = TraverseOpts::default();
                let mut prompt = Prompt::new(
                    PromptAnswer::all() & !PromptAnswer::DIFF,
                    PromptFlags::new(self.assume_no, self.assume_yes, self.list),
                );

                // traverse all runner profiles
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    if let ProfileKind::Runner(runner) = ctx.item.kind() {
                        let this_run_dir = run_dir.join(ctx.item.id_or(ctx.name))?;
                        runner.resolve(&this_run_dir, |path, policy| {
                            // filter entries with skip policy
                            if policy == RunnerPolicy::Skip {
                                return Ok(());
                            }

                            // prompt user
                            let relpath = path.to_rel(run_dir)?;
                            outln!("{}", relpath.display().style(CliContext::OUTPUT_PATH));
                            let msg = "Do you really want to run the script?";
                            let paths = &[&path];
                            let action = || {
                                if let exit_status = Command::new(path.path())
                                    .stdin(if interactive {
                                        Stdio::inherit()
                                    } else {
                                        Stdio::null()
                                    })
                                    .status()
                                    .context("Script failed to run")?
                                    .code()
                                    && exit_status != Some(0)
                                {
                                    if let Some(status) = exit_status {
                                        bail!(format!("Script failed with status code {status}"))
                                    } else {
                                        bail!("Script was terminated by a signal")
                                    }
                                }

                                Ok(())
                            };

                            // handle flags
                            prompt.handled_prompt(msg, paths, action)?;
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
