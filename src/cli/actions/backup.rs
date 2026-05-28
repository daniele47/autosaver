use std::collections::HashSet;

use indexmap::IndexMap;

use crate::{
    cli::{
        Cli,
        ctx::{CliContext, Paths},
        prompt::{Prompt, PromptAnswer, PromptFlags},
    },
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{
        ProfileKind, TraverseOpts,
        module::{Module, ModuleEntry},
    },
};

type Entries<'a> = IndexMap<RelPathStr, (&'a ModuleEntry, [Option<AbsPathStr>; 2])>;

fn resolve<'a>(runner: &'a Module, dirs: &[&AbsPathStr; 2]) -> anyhow::Result<Entries<'a>> {
    let entries = <Entries>::new();
    for entry in runner.entries() {
        println!("{dirs:?} {entry:?}");
    }

    Ok(entries)
}

impl Cli {
    pub fn action_backup(&self, ctx: &CliContext) -> anyhow::Result<()> {
        let home_dir = &ctx.paths[&Paths::Home];
        let backup_dir = &ctx.paths[&Paths::Backup];
        let trav_opts = TraverseOpts::default();
        let all_paths = HashSet::<RelPathStr>::new();
        let prompt = Prompt::new(
            PromptAnswer::all(),
            PromptFlags::new(self.assume_no, self.assume_yes, self.list),
        );

        //
        ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
            if let ProfileKind::Module(module) = ctx.item.kind() {
                CliContext::output_profile(ctx.name, CliContext::OUTPUT_PROFILE);
                let this_backup_dir = backup_dir.join(ctx.item.id_or(ctx.name))?;
                for (path, entry) in resolve(module, &[home_dir, &this_backup_dir])? {
                    // TODO
                    println!("{path:?} {entry:?} {all_paths:?} {prompt:?}");
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}
