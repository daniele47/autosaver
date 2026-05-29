use std::collections::HashSet;

use anyhow::bail;
use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
        prompt::{Prompt, PromptAnswer, PromptFlags},
    },
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{
        ProfileKind, TraverseOpts,
        module::{Module, ModuleEntry, ModulePolicy},
    },
};

type Entries<'a> = IndexMap<RelPathStr, (&'a ModuleEntry, [Option<AbsPathStr>; 2])>;

fn resolve<'a>(runner: &'a Module, dirs: &[&AbsPathStr; 2]) -> anyhow::Result<Entries<'a>> {
    let mut entries = <Entries>::new();
    for entry in runner.entries() {
        for (i, dir) in dirs.iter().enumerate() {
            for p in entry.path().to_abs(dir)?.all_files_ord()? {
                let rp = p.to_rel(dir)?;
                match entries.entry(rp) {
                    Entry::Vacant(e) => {
                        let mut val = (entry, [None, None]);
                        val.1[i] = Some(p);
                        e.insert(val);
                    }
                    Entry::Occupied(mut e) => {
                        if e.get().1[i].as_ref().is_none_or(|_| {
                            (*entry.policy() as u64) < (*(e.get()).0.policy() as u64)
                        }) {
                            e.get_mut().1[i] = Some(p);
                        }
                    }
                }
            }
        }
    }

    Ok(entries)
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

        // traverse profiles
        ctx.profiles.traverse(&ctx.curr_profile, trav_opts, |ctx| {
            if let ProfileKind::Module(module) = ctx.item.kind() {
                CliContext::output_profile(ctx.name, CliContext::OUTPUT_PROFILE);
                let this_backup_dir = backup_dir.join(ctx.item.id_or(ctx.name))?;
                for (path, entry) in resolve(module, &[home_dir, &this_backup_dir])? {
                    // filter entries with skip policy
                    if *entry.0.policy() == ModulePolicy::Ignore {
                        continue;
                    }

                    // check path was not found yet
                    if all_paths.contains(&path) {
                        let p = path.display();
                        bail!("Path '{p}' was already found previously");
                    }

                    // run action
                    match &self.cmd {
                        // delete action
                        CliCmd::Delete {
                            only_original,
                            only_backup,
                        } => {
                            let mut path_shown = false;
                            if (*only_original || !only_backup)
                                && let Some(original_file) = &entry.1[0]
                            {
                                let msg = "Do you really want to delete original file?";
                                let paths = &[original_file];
                                let action = || original_file.purge_path();
                                CliContext::output_path(&path, CliContext::OUTPUT_PATH);
                                path_shown = true;
                                prompt.handled_prompt_available(msg, paths, action)?;
                            }
                            if (*only_backup || !only_original)
                                && let Some(backup_file) = &entry.1[1]
                            {
                                let msg = "Do you really want to delete backup file?";
                                let paths = &[backup_file];
                                let action = || backup_file.purge_path();
                                if !path_shown {
                                    CliContext::output_path(&path, CliContext::OUTPUT_PATH);
                                }
                                prompt.handled_prompt_available(msg, paths, action)?;
                            }
                        }
                        // backup action
                        CliCmd::List { act_backup }
                        | CliCmd::Save { act_backup }
                        | CliCmd::Restore { act_backup } => {
                            let _ = act_backup;
                        }
                        _ => unreachable!("Invalid backup action"),
                    }

                    // insert path to all paths
                    all_paths.insert(path);
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}
