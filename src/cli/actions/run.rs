use std::process::{Command, Stdio};

use anyhow::{Context, bail};
use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{
        Cli, CliCmd,
        config::{CliContext, Paths},
    },
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{
        ProfileKind,
        runner::{Runner, RunnerEntry, RunnerPolicy},
    },
    warning,
};

fn resolve<'a>(
    runner: &'a Runner,
    dir: &AbsPathStr,
) -> anyhow::Result<IndexMap<AbsPathStr, &'a RunnerEntry>> {
    let mut entries = IndexMap::<AbsPathStr, &'a RunnerEntry>::new();
    for entry in &runner.entries {
        for p in entry.path.to_abs(dir)?.all_files_ord()? {
            match entries.entry(p) {
                Entry::Occupied(mut e) => {
                    if (entry.policy as u64) < ((e.get()).policy as u64) {
                        e.insert(entry);
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(entry);
                }
            }
        }
    }
    Ok(entries)
}

impl Cli {
    pub fn action_run(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run { allow_stdin: stdin } => {
                let run_dir = &ctx.paths[&Paths::Run];

                // traverse all runner profiles
                ctx.profiles.traverse(&ctx.curr_profile, |trav_ctx| {
                    if let ProfileKind::Runner(runner) = &trav_ctx.item.kind {
                        ctx.col.output_profile(trav_ctx.name);
                        if self.choice {
                            let mut execute = false;
                            let msg = "Do you want to execute this profile?";
                            let mut name_str = trav_ctx.name.to_string_lossy().to_string();
                            name_str.push_str(".conf");
                            let config_path = RelPathStr::try_from(name_str)?;
                            let paths = &[&ctx.paths[&Paths::Config].join(&config_path)?];
                            let action = || {
                                execute = true;
                                Ok(())
                            };
                            ctx.prompt.question(msg, paths, action, &ctx.col)?;
                            if !execute {
                                return Ok(());
                            }
                        }
                        let this_run_dir = run_dir.join(trav_ctx.item.id_or(trav_ctx.name))?;
                        for (path, entry) in resolve(runner, &this_run_dir)? {
                            // filter entries with skip policy
                            if entry.policy == RunnerPolicy::Exclude {
                                continue;
                            }

                            // output path
                            let relpath = path.to_rel(run_dir)?;
                            ctx.col.output_path(&relpath, ctx.col.output_path);

                            // prompt user
                            let msg = if entry.stdin {
                                if stdin {
                                    "Do you really want to run the script with stdin enabled?"
                                } else {
                                    warning!("Script requires stdin to run");
                                    continue;
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
                            ctx.prompt.question(msg, paths, action, &ctx.col)?;
                        }
                    }
                    Ok(())
                })
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
