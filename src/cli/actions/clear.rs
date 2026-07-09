use indexmap::{IndexMap, map::Entry};

use crate::{
    cli::{
        Cli, CliCmd,
        config::{CliContext, Paths},
    },
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{ProfileKind, module::ModulePolicy},
    warning,
};

fn resolve<'a>(
    relpaths: impl Iterator<Item = (&'a RelPathStr, bool)>,
    dir: &AbsPathStr,
    entries: &mut IndexMap<AbsPathStr, bool>,
) -> anyhow::Result<()> {
    for (path, ignored) in relpaths {
        for p in path.to_abs(dir)?.all_files_ord()? {
            let p = if p.is_file() {
                p.canonicalize()?
            } else {
                continue;
            };
            match entries.entry(p) {
                Entry::Occupied(mut e) => {
                    e.insert(*e.get() || ignored);
                }
                Entry::Vacant(e) => {
                    e.insert(ignored);
                }
            };
        }
    }
    Ok(())
}

impl Cli {
    pub fn action_clear(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match &self.cmd {
            CliCmd::Clear { act_delsymlinks } => {
                let run_dir = &ctx.paths[&Paths::Run];
                let backup_dir = &ctx.paths[&Paths::Backup];
                let root_dir = &ctx.paths[&Paths::Root];
                let mut entries = IndexMap::new();

                // traverse all leaf profiles
                ctx.profiles.traverse(&ctx.root_profile, |ctx| {
                    match ctx.item.kind() {
                        ProfileKind::Module(module) => {
                            let this_backup_dir = backup_dir.join(ctx.item.id_or(ctx.name))?;
                            let relpaths = module
                                .entries()
                                .iter()
                                .map(|e| (e.path(), e.policy() == &ModulePolicy::Ignore));
                            resolve(relpaths, &this_backup_dir, &mut entries)?;
                        }
                        ProfileKind::Runner(runner) => {
                            let this_runner_dir = run_dir.join(ctx.item.id_or(ctx.name))?;
                            let relpaths = runner.entries().iter().map(|e| (e.path(), false));

                            resolve(relpaths, &this_runner_dir, &mut entries)?;
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
                        if let Some(ignored) = entries.get(&file) {
                            if *ignored {
                                let relpath = file.to_rel(root_dir)?;
                                ctx.col.output_path(&relpath, ctx.col.output_path);
                                if !act_delsymlinks.allow_symlink
                                    && file.path().symlink_metadata()?.is_symlink()
                                {
                                    warning!("Symlink flag is required to delete symlinks")
                                } else {
                                    ctx.prompt.question(
                                        "Do you really want to delete ignored file?",
                                        &[&file],
                                        || file.purge_path(),
                                        &ctx.col,
                                    )?;
                                }
                            }
                        } else {
                            let relpath = file.to_rel(root_dir)?;
                            ctx.col.output_path(&relpath, ctx.col.output_path);
                            if !act_delsymlinks.allow_symlink
                                && file.path().symlink_metadata()?.is_symlink()
                            {
                                warning!("Symlink flag is required to delete symlinks")
                            } else {
                                ctx.prompt.question(
                                    "Do you really want to delete untracked file?",
                                    &[&file],
                                    || file.purge_path(),
                                    &ctx.col,
                                )?;
                            }
                        }
                    }
                }

                Ok(())
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
