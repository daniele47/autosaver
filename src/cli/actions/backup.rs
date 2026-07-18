use std::collections::HashSet;

use anyhow::bail;
use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{
        Cli, CliCmd,
        config::{CliContext, Paths},
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
    for entry in &runner.entries {
        for (i, dir) in dirs.iter().enumerate() {
            for p in entry.path.to_abs(dir)?.all_files_ord()? {
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
                        if (entry.policy as u64) < ((e.get()).0.policy as u64) {
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

        // traverse profiles
        ctx.profiles.traverse(&ctx.curr_profile, |trav_ctx| {
            if let ProfileKind::Module(module) = &trav_ctx.item.kind {
                Self::output_profile(trav_ctx.name, ctx.col.output_profile);
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
                let this_backup_dir = backup_dir.join(trav_ctx.item.id_or(trav_ctx.name))?;

                // cleanup paths before running anything else
                if !module.cleanup.is_empty() {
                    let mut allow_symlinks = false;
                    let mut allow_purge = false;
                    let run_cleanup = match self.cmd {
                        CliCmd::Restore {
                            act_delsymlinks,
                            allow_cleanup,
                            act_saverestore,
                            ..
                        } => {
                            allow_symlinks = act_delsymlinks.allow_symlink;
                            allow_purge = act_saverestore.allow_purge;
                            allow_cleanup
                        }
                        CliCmd::Delete {
                            only_cleanup,
                            only_backup,
                            only_original,
                            act_delsymlinks,
                        } => {
                            allow_symlinks = act_delsymlinks.allow_symlink;
                            only_cleanup || (!only_backup && !only_original)
                        }
                        _ => false,
                    };
                    if run_cleanup {
                        for cleanup in &module.cleanup {
                            let abs_cleanup = cleanup.to_abs(home_dir)?;
                            if abs_cleanup.path().symlink_metadata().is_err() {
                                continue;
                            }
                            Self::output_path(&abs_cleanup, ctx.col.output_cleanup);
                            let is_symlink = abs_cleanup
                                .path()
                                .symlink_metadata()
                                .is_ok_and(|d| d.is_symlink());
                            if !allow_purge {
                                warning!("{} flag is required to cleanup files", Self::PURGE_FLAG);
                            } else if !allow_symlinks && is_symlink {
                                warning!(
                                    "{} flag is required to cleanup symlink files",
                                    Self::SYMLINK_FLAG
                                );
                            } else {
                                ctx.prompt.question(
                                    "Do you really want to delete cleanup path?",
                                    &[&abs_cleanup],
                                    || abs_cleanup.purge_path_opts(true),
                                    &ctx.col,
                                )?;
                            }
                        }
                    }
                }

                for (path, entry) in resolve(module, &[home_dir, &this_backup_dir])? {
                    // filter entries with skip policy
                    if entry.0.policy == ModulePolicy::Exclude {
                        continue;
                    }

                    // check path was not found yet
                    match &self.cmd {
                        CliCmd::Restore {
                            act_saverestore, ..
                        }
                        | CliCmd::Save {
                            act_saverestore, ..
                        } => {
                            let relpath = if let Some(original_file) = &entry.1[0] {
                                original_file.to_rel(home_dir)
                            } else if let Some(backup_file) = &entry.1[1] {
                                backup_file.to_rel(backup_dir)
                            } else {
                                bail!("missing path")
                            };
                            if let Ok(relpath) = relpath
                                && !all_paths.insert(relpath.clone())
                            {
                                let p = relpath.display();
                                let msg = format!("Path '{p}' was already found previously");
                                if act_saverestore.allow_duplicates {
                                    warning!("{msg}")
                                } else {
                                    bail!(msg)
                                }
                            }
                        }
                        _ => {}
                    }

                    // run action
                    match &self.cmd {
                        // delete action
                        CliCmd::Delete {
                            only_cleanup,
                            only_original,
                            only_backup,
                            act_delsymlinks,
                        } => {
                            let mut path_printed = false;
                            let no_filter = !only_backup && !only_original && !only_cleanup;
                            if (*only_original || no_filter)
                                && let Some(original_file) = &entry.1[0]
                            {
                                path_printed = true;
                                Self::output_path(&path, ctx.col.output_path);
                                if !act_delsymlinks.allow_symlink
                                    && original_file.path().symlink_metadata()?.is_symlink()
                                {
                                    warning!(
                                        "{} flag is required to delete \
                                        symlink in home directory",
                                        Self::SYMLINK_FLAG
                                    )
                                } else {
                                    ctx.prompt.question(
                                        "Do you really want to delete home file?",
                                        &[original_file],
                                        || original_file.purge_path(),
                                        &ctx.col,
                                    )?;
                                }
                            }
                            if (*only_backup || no_filter)
                                && let Some(backup_file) = &entry.1[1]
                            {
                                if !path_printed {
                                    Self::output_path(&path, ctx.col.output_path);
                                }
                                if !act_delsymlinks.allow_symlink
                                    && backup_file.path().symlink_metadata()?.is_symlink()
                                {
                                    warning!(
                                        "{} flag is required to delete \
                                         symlink in backup directory",
                                        Self::SYMLINK_FLAG
                                    );
                                } else {
                                    ctx.prompt.question(
                                        "Do you really want to delete backup file?",
                                        &[backup_file],
                                        || backup_file.purge_path(),
                                        &ctx.col,
                                    )?;
                                }
                            }
                        }
                        // backup action
                        CliCmd::List { act_backup }
                        | CliCmd::Save { act_backup, .. }
                        | CliCmd::Restore { act_backup, .. } => match &entry.1 {
                            // file is missing in the backup
                            [Some(p1), None] => match &self.cmd {
                                CliCmd::Save { .. } => {
                                    Self::output_path(&path, ctx.col.output_create);
                                    ctx.prompt.question(
                                        "Do you really want to create backup file?",
                                        &[p1],
                                        || p1.copy_file(&path.to_abs(&this_backup_dir)?),
                                        &ctx.col,
                                    )?;
                                }
                                CliCmd::Restore {
                                    act_delsymlinks,
                                    act_saverestore,
                                    ..
                                } => {
                                    Self::output_path(&path, ctx.col.output_delete);
                                    if !act_saverestore.allow_purge {
                                        warning!(
                                            "{} flag is required to delete \
                                                files in home directory",
                                            Self::PURGE_FLAG
                                        );
                                    } else if !act_delsymlinks.allow_symlink
                                        && p1.path().symlink_metadata()?.is_symlink()
                                    {
                                        warning!(
                                            "{} flag is required to delete \
                                                symlinks in home directory",
                                            Self::SYMLINK_FLAG
                                        )
                                    } else {
                                        ctx.prompt.question(
                                            "Do you really want to delete home file?",
                                            &[p1],
                                            || p1.purge_path(),
                                            &ctx.col,
                                        )?;
                                    }
                                }
                                CliCmd::List { .. } => {
                                    Self::output_path(&path, ctx.col.output_missing);
                                }
                                _ => unreachable!("must either save or restore or list"),
                            },
                            // file is missing in home
                            [None, Some(p1)] => match &self.cmd {
                                CliCmd::Save {
                                    act_delsymlinks,
                                    act_saverestore,
                                    ..
                                } => {
                                    Self::output_path(&path, ctx.col.output_missing);
                                    if !act_saverestore.allow_purge {
                                        warning!(
                                            "{} flag is required to delete \
                                                files in backup directory",
                                            Self::PURGE_FLAG
                                        );
                                    } else if !act_delsymlinks.allow_symlink
                                        && p1.path().symlink_metadata()?.is_symlink()
                                    {
                                        warning!(
                                            "{} flag is required to delete \
                                                symlinks in backup directory",
                                            Self::SYMLINK_FLAG
                                        )
                                    } else {
                                        ctx.prompt.question(
                                            "Do you really want to delete backup file?",
                                            &[p1],
                                            || p1.purge_path(),
                                            &ctx.col,
                                        )?;
                                    }
                                }
                                CliCmd::Restore { .. } => {
                                    Self::output_path(&path, ctx.col.output_create);
                                    ctx.prompt.question(
                                        "Do you really want to create home file?",
                                        &[p1],
                                        || p1.copy_file(&path.to_abs(home_dir)?),
                                        &ctx.col,
                                    )?;
                                }
                                CliCmd::List { .. } => {
                                    Self::output_path(&path, ctx.col.output_missing);
                                }
                                _ => unreachable!("must either save or restore or list"),
                            },
                            // files differ
                            [Some(p1), Some(p2)] if !p1.files_eq(p2) => {
                                if entry.0.policy == ModulePolicy::NotDiff
                                    && !act_backup.show_excluded
                                {
                                    continue;
                                }
                                Self::output_path(&path, ctx.col.output_diff);
                                if matches!(&self.cmd, CliCmd::Save { .. }) {
                                    let msg = "Do you really want to update backup file?";
                                    let paths = &[p2, p1];
                                    let action = || p1.copy_file(p2);
                                    ctx.prompt.question(msg, paths, action, &ctx.col)?;
                                } else if matches!(&self.cmd, CliCmd::Restore { .. }) {
                                    let msg = "Do you really want to update home file?";
                                    let paths = &[p1, p2];
                                    let action = || p2.copy_file(p1);
                                    ctx.prompt.question(msg, paths, action, &ctx.col)?;
                                }
                            }
                            // files are equal
                            [Some(p1), Some(p2)] => {
                                if act_backup.show_unmodified {
                                    Self::output_path(&path, ctx.col.output_unmodified);
                                    let msg = "Nothing to be done. Type y/n to continue...";
                                    let action = || Ok(());
                                    if matches!(&self.cmd, CliCmd::Save { .. }) {
                                        ctx.prompt.question(msg, &[p2, p1], action, &ctx.col)?;
                                    } else if matches!(&self.cmd, CliCmd::Restore { .. }) {
                                        ctx.prompt.question(msg, &[p1, p2], action, &ctx.col)?;
                                    }
                                }
                            }
                            _ => unreachable!("Invalid files"),
                        },
                        _ => unreachable!("Invalid backup action"),
                    }
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}
