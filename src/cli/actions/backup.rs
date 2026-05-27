use std::collections::BTreeSet;

use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{Cli, ctx::CliContext},
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::module::{Module, ModuleEntry},
};

fn resolve<'a>(
    runner: &'a Module,
    dirs: &[&AbsPathStr],
) -> anyhow::Result<IndexMap<RelPathStr, &'a ModuleEntry>> {
    let mut elems: IndexMap<RelPathStr, &ModuleEntry> = IndexMap::new();

    for entry in runner.entries() {
        let mut all_files_ord = BTreeSet::default();
        for dir in dirs {
            all_files_ord.extend(
                entry
                    .path()
                    .to_abs(dir)?
                    .all_files_ord()?
                    .iter()
                    .map(|p| p.to_rel(dir).expect("abs path cannot to relative")),
            );
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
        Ok(())
    }
}
