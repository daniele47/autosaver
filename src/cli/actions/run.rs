use std::{
    collections::HashSet,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
        prompt::{Prompt, PromptAnswer, PromptFlags},
    },
    fs::abs::AbsPathStr,
    prof::{
        ProfileKind, TraverseOpts,
        runner::{Runner, RunnerEntry, RunnerPolicy},
    },
    warning,
};

fn resolve<'a>(
    runner: &'a Runner,
    dir: &AbsPathStr,
    entries: &mut IndexMap<AbsPathStr, *const RunnerEntry>,
) -> anyhow::Result<()> {
    let mut all = vec![];

    for entry in runner.entries() {
        entry.path().to_abs(dir)?.all_files_ord(&mut all)?;
        for p in all.drain(..) {
            match entries.entry(p) {
                Entry::Occupied(mut e) => unsafe {
                    if (*entry.policy() as u64) < (*(**e.get()).policy() as u64) {
                        e.insert(entry);
                    }
                },
                Entry::Vacant(e) => {
                    e.insert(entry);
                }
            }
        }
        all.clear();
    }

    Ok(())
}

impl Cli {
    pub fn action_run(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run { stdin } => {
                let run_dir = &ctx.paths[&Paths::Run];
                let trav_opts = TraverseOpts::default();
                let mut all_paths = HashSet::<AbsPathStr>::new();
                let mut entries = IndexMap::new();
                let prompt = Prompt::new(
                    PromptAnswer::all() & !PromptAnswer::DIFF,
                    PromptFlags::new(self.assume_no, self.assume_yes, self.list),
                );

                // traverse all runner profiles
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    if let ProfileKind::Runner(runner) = ctx.item.kind() {
                        CliContext::output_profile(ctx.name, CliContext::OUTPUT_PROFILE);
                        let this_run_dir = run_dir.join(ctx.item.id_or(ctx.name))?;
                        resolve(runner, &this_run_dir, &mut entries)?;
                        for (path, entry) in entries.drain(..) {
                            let entry = unsafe { &*entry };
                            // filter entries with skip policy
                            if *entry.policy() == RunnerPolicy::Skip {
                                return Ok(());
                            }

                            // prompt user
                            let relpath = path.to_rel(run_dir)?;

                            // check path was not found yet
                            if all_paths.contains(&path) {
                                let p = path.to_rel(run_dir)?;
                                let p = p.display();
                                bail!("Script '{p}' was already run previously");
                            }

                            // output path
                            CliContext::output_path(&relpath, CliContext::OUTPUT_PATH);

                            // handle flags
                            let msg = if entry.stdin() {
                                if stdin {
                                    "Do you really want to run the script with stdin enabled?"
                                } else {
                                    warning!("Script requires stdin to run");
                                    return Ok(());
                                }
                            } else {
                                "Do you really want to run the script?"
                            };
                            let paths = &[&path];
                            let action = || {
                                if let exit_status = Command::new(path.path())
                                    .stdin(if stdin {
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
                            prompt.handled_prompt(msg, paths, action)?;

                            // insert path to all paths
                            all_paths.insert(path);
                        }
                    }
                    Ok(())
                })
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
