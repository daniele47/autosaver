use std::collections::HashSet;

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
        module::{Module, ModulePolicy},
        runner::{Runner, RunnerPolicy},
    },
};

fn resolve_runner(
    runner: &Runner,
    dir: &AbsPathStr,
    entries: &mut IndexMap<AbsPathStr, bool>,
) -> anyhow::Result<()> {
    for entry in runner.entries() {
        for p in entry.path().to_abs(dir)?.all_files_ord()? {
            let new = *entry.policy() != RunnerPolicy::Skip;
            match entries.entry(p) {
                Entry::Occupied(mut e) => {
                    if *e.get() && !new {
                        e.insert(new);
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(new);
                }
            };
        }
    }
    Ok(())
}
fn resolve_module(
    runner: &Module,
    dir: &AbsPathStr,
    entries: &mut IndexMap<AbsPathStr, bool>,
) -> anyhow::Result<()> {
    for entry in runner.entries() {
        for p in entry.path().to_abs(dir)?.all_files_ord()? {
            let new = *entry.policy() != ModulePolicy::Ignore;
            match entries.entry(p) {
                Entry::Occupied(mut e) => {
                    if *e.get() && !new {
                        e.insert(new);
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(new);
                }
            };
        }
    }
    Ok(())
}

impl Cli {
    pub fn action_clear(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Run { stdin } => {
                let run_dir = &ctx.paths[&Paths::Run];
                let trav_opts = TraverseOpts::default();
                let mut all_paths = HashSet::<AbsPathStr>::new();
                let prompt = Prompt::new(
                    PromptAnswer::all() & !PromptAnswer::DIFF,
                    PromptFlags::new(self.assume_no, self.assume_yes, self.list),
                );

                // traverse all runner profiles
                ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
                    if let ProfileKind::Runner(runner) = ctx.item.kind() {}
                    Ok(())
                })
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
