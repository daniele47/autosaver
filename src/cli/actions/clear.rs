use indexmap::IndexSet;

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
        prompt::{Prompt, PromptAnswer, PromptFlags},
    },
    fs::abs::AbsPathStr,
    prof::{ProfileKind, TraverseOpts, module::Module, runner::Runner},
};

fn resolve_runner(
    runner: &Runner,
    dir: &AbsPathStr,
    entries: &mut IndexSet<AbsPathStr>,
) -> anyhow::Result<()> {
    for entry in runner.entries() {
        for p in entry.path().to_abs(dir)?.all_files_ord()? {
            let p = if p.is_file() {
                p.canonicalize()?
            } else {
                continue;
            };
            entries.insert(p);
        }
    }
    Ok(())
}
fn resolve_module(
    module: &Module,
    dir: &AbsPathStr,
    entries: &mut IndexSet<AbsPathStr>,
) -> anyhow::Result<()> {
    for entry in module.entries() {
        for p in entry.path().to_abs(dir)?.all_files_ord()? {
            let p = if p.is_file() {
                p.canonicalize()?
            } else {
                continue;
            };
            entries.insert(p);
        }
    }
    Ok(())
}

impl Cli {
    pub fn action_clear(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Clear => {
                let run_dir = &ctx.paths[&Paths::Run];
                let backup_dir = &ctx.paths[&Paths::Backup];
                let root_dir = &ctx.paths[&Paths::Root];
                let trav_opts = TraverseOpts::default();
                let mut entries = IndexSet::new();
                let prompt = Prompt::new(
                    PromptAnswer::all() & !PromptAnswer::DIFF,
                    PromptFlags::new(self.assume_no, self.assume_yes, self.list),
                );

                // traverse all leaf profiles
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    match ctx.item.kind() {
                        ProfileKind::Module(module) => {
                            let this_backup_dir = backup_dir.join(ctx.item.id_or(ctx.name))?;
                            resolve_module(module, &this_backup_dir, &mut entries)?;
                        }
                        ProfileKind::Runner(runner) => {
                            let this_runner_dir = run_dir.join(ctx.item.id_or(ctx.name))?;
                            resolve_runner(runner, &this_runner_dir, &mut entries)?;
                        }
                        _ => {}
                    }
                    Ok(())
                })?;

                // compare found files with those tracked
                for dir in [run_dir, backup_dir] {
                    for file in dir.to_owned().all_files_ord()? {
                        let file = if file.is_file() {
                            file.canonicalize()?
                        } else {
                            continue;
                        };
                        if !entries.contains(&file) {
                            let relpath = file.to_rel(root_dir)?;
                            CliContext::output_path(relpath, CliContext::OUTPUT_PATH);
                            prompt.handled_prompt_available(
                                "Do you really want to delete untracked file?",
                                &[&file],
                                || file.purge_path(),
                            )?;
                        }
                    }
                }

                Ok(())
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
