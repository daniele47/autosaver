use std::collections::HashSet;

use anyhow::bail;
use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{
        Cli, CliCmd,
        config::{CliContext, Paths},
        prompt::{Prompt, PromptAnswer, PromptFlags},
    },
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{
        ProfileKind,
        module::{Module, ModuleEntry, ModulePolicy},
    },
    warning,
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
                        if e.get().1[i].is_none() {
                            e.get_mut().1[i] = Some(p);
                        }
                        if (*entry.policy() as u64) < (*(e.get()).0.policy() as u64) {
                            e.get_mut().0 = entry;
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
        let mut all_paths = HashSet::<RelPathStr>::new();
        let prompt = Prompt::new(
            PromptAnswer::all(),
            PromptFlags::new(self.assume_no, self.assume_yes, self.list),
            &ctx.col,
        );

        // traverse profiles
        ctx.profiles.traverse(&ctx.curr_profile, |trav_ctx| {
            if let ProfileKind::Module(module) = trav_ctx.item.kind() {
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
                    prompt.handled_prompt_available(msg, paths, action)?;
                    if !execute {
                        return Ok(());
                    }
                }
                let this_backup_dir = backup_dir.join(trav_ctx.item.id_or(trav_ctx.name))?;
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
                            let mut path_printed = false;
                            if (*only_original || !only_backup)
                                && let Some(original_file) = &entry.1[0]
                            {
                                path_printed = true;
                                ctx.col.output_path(&path, ctx.col.output_path);
                                if !self.symlink
                                    && original_file.path().symlink_metadata()?.is_symlink()
                                {
                                    warning!(
                                        "Symlink flag is required to delete \
                                        symlink in home directory"
                                    )
                                } else {
                                    prompt.handled_prompt_available(
                                        "Do you really want to delete home file?",
                                        &[original_file],
                                        || original_file.purge_path(),
                                    )?;
                                }
                            }
                            if (*only_backup || !only_original)
                                && let Some(backup_file) = &entry.1[1]
                            {
                                if !path_printed {
                                    ctx.col.output_path(&path, ctx.col.output_path);
                                }
                                if !self.symlink
                                    && backup_file.path().symlink_metadata()?.is_symlink()
                                {
                                    warning!(
                                        "Symlink flag is required to delete \
     symlink in backup directory"
                                    );
                                } else {
                                    prompt.handled_prompt_available(
                                        "Do you really want to delete backup file?",
                                        &[backup_file],
                                        || backup_file.purge_path(),
                                    )?;
                                }
                            }
                        }
                        // backup action
                        CliCmd::List { act_backup }
                        | CliCmd::Save { act_backup, .. }
                        | CliCmd::Restore { act_backup, .. } => match &entry.1 {
                            // file is missing in the backup
                            [Some(p1), None] => {
                                ctx.col.output_path(&path, ctx.col.output_missing);
                                match &self.cmd {
                                    CliCmd::Save { .. } => {
                                        prompt.handled_prompt_available(
                                            "Do you really want to create backup file?",
                                            &[p1],
                                            || p1.copy_file(&path.to_abs(&this_backup_dir)?),
                                        )?;
                                    }
                                    CliCmd::Restore { force, .. } => {
                                        if !force {
                                            warning!(
                                                "Force flag is required to delete \
                                                files in home directory"
                                            );
                                        } else if !self.symlink
                                            && p1.path().symlink_metadata()?.is_symlink()
                                        {
                                            warning!(
                                                "Symlink flag is required to delete \
                                                symlinks in home directory"
                                            )
                                        } else {
                                            prompt.handled_prompt_available(
                                                "Do you really want to delete home file?",
                                                &[p1],
                                                || p1.purge_path(),
                                            )?;
                                        }
                                    }
                                    CliCmd::List { .. } => {}
                                    _ => unreachable!("must either save or restore or list"),
                                }
                            }
                            // file is missing in home
                            [None, Some(p1)] => {
                                ctx.col.output_path(&path, ctx.col.output_missing);
                                match &self.cmd {
                                    CliCmd::Save { force, .. } => {
                                        if !force {
                                            warning!(
                                                "Force flag is required to delete \
                                                files in backup directory"
                                            );
                                        } else if !self.symlink
                                            && p1.path().symlink_metadata()?.is_symlink()
                                        {
                                            warning!(
                                                "Symlink flag is required to delete \
                                                symlinks in backup directory"
                                            )
                                        } else {
                                            prompt.handled_prompt_available(
                                                "Do you really want to delete backup file?",
                                                &[p1],
                                                || p1.purge_path(),
                                            )?;
                                        }
                                    }
                                    CliCmd::Restore { .. } => {
                                        prompt.handled_prompt_available(
                                            "Do you really want to create home file?",
                                            &[p1],
                                            || p1.copy_file(&path.to_abs(home_dir)?),
                                        )?;
                                    }
                                    CliCmd::List { .. } => {}
                                    _ => unreachable!("must either save or restore or list"),
                                }
                            }
                            // files differ
                            [Some(p1), Some(p2)] if !p1.files_eq(p2) => {
                                if *entry.0.policy() == ModulePolicy::NotDiff && !act_backup.all {
                                    continue;
                                }
                                ctx.col.output_path(&path, ctx.col.output_diff);
                                if matches!(&self.cmd, CliCmd::Save { .. }) {
                                    let msg = "Do you really want to update backup file?";
                                    let paths = &[p2, p1];
                                    let action = || p1.copy_file(p2);
                                    prompt.handled_prompt_available(msg, paths, action)?;
                                } else if matches!(&self.cmd, CliCmd::Restore { .. }) {
                                    let msg = "Do you really want to update home file?";
                                    let paths = &[p1, p2];
                                    let action = || p2.copy_file(p1);
                                    prompt.handled_prompt_available(msg, paths, action)?;
                                }
                            }
                            // files are equal
                            [Some(p1), Some(p2)] => {
                                if act_backup.unmodified {
                                    ctx.col.output_path(&path, ctx.col.output_unmodified);
                                    let msg = "Nothing to be done. Type y/n to continue...";
                                    let action = || Ok(());
                                    if matches!(&self.cmd, CliCmd::Save { .. }) {
                                        prompt.handled_prompt_available(msg, &[p2, p1], action)?;
                                    } else if matches!(&self.cmd, CliCmd::Restore { .. }) {
                                        prompt.handled_prompt_available(msg, &[p1, p2], action)?;
                                    }
                                }
                            }
                            _ => unreachable!("Invalid files"),
                        },
                        _ => unreachable!("Invalid backup action"),
                    }

                    // insert path to all paths
                    if !self.list && !matches!(self.cmd, CliCmd::List { .. }) {
                        all_paths.insert(path);
                    }
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}
