use std::collections::{BTreeSet, HashSet};

use indexmap::{IndexMap, map::Entry};

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

fn resolve<'a>(
    runner: &'a Module,
    dirs: &[&AbsPathStr],
) -> anyhow::Result<IndexMap<RelPathStr, &'a ModuleEntry>> {
    let mut elems: IndexMap<RelPathStr, &ModuleEntry> = IndexMap::new();

    for entry in runner.entries() {
        let mut all_files_ord = BTreeSet::default();
        for dir in dirs {
            // all_files_ord.extend(
            //     entry
            //         .path()
            //         .to_abs(dir)?
            //         .all_files_ord()?
            //         .iter()
            //         .map(|p| p.to_rel(dir).expect("abs path cannot to relative")),
            // );
        }
        all_files_ord.into_iter().try_for_each(|p| {
            match elems.entry(p) {
                Entry::Occupied(mut e) => {
                    if (*entry.policy() as u64) < (*e.get().policy() as u64) {
                        e.insert(entry);
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(entry);
                }
            }
            anyhow::Ok(())
        })?;
    }

    Ok(elems)
}

impl Cli {
    pub fn action_backup(&self, ctx: &CliContext) -> anyhow::Result<()> {
        let home_dir = &ctx.paths[&Paths::Home];
        let backup_dir = &ctx.paths[&Paths::Backup];
        let trav_opts = TraverseOpts::default();
        let mut all_paths = HashSet::<RelPathStr>::new();
        let prompt = Prompt::new(
            PromptAnswer::all(),
            PromptFlags::new(self.assume_no, self.assume_yes, self.list),
        );

        //
        ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
            if let ProfileKind::Module(module) = ctx.item.kind() {
                CliContext::output_profile(ctx.name, CliContext::OUTPUT_PROFILE);
                let this_backup_dir = backup_dir.join(ctx.item.id_or(ctx.name))?;
                for (path, entry) in resolve(module, &[&home_dir, &this_backup_dir])? {
                    // TODO
                    println!("{}", path.display());
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}
