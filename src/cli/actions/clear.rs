use std::collections::{BTreeSet, HashSet};

use crate::{
    cli::{actions::Runner, error::Result, flags::Flag},
    core::{
        fs::AbsPath,
        profile::{ProfileType, composite::ProfileLoader},
    },
    debug,
};

fn clear_paths(
    runner: &Runner,
    all_paths: BTreeSet<AbsPath>,
    tracked_paths: &HashSet<AbsPath>,
    flag_list: bool,
) -> Result<()> {
    for path in all_paths {
        let canon = path.canonicalize()?;
        let rel_path = canon.to_relative(&runner.paths("root")?)?;
        let rel_path_str = rel_path.to_str_lossy();
        if !tracked_paths.contains(&canon) {
            runner
                .inout
                .writeln(rel_path_str, Runner::PATH_UNTRACKED_COL);
            if !flag_list {
                runner.prompt("Do you want to delete the untracked file?", |_| {
                    Ok(path.purge_path(false)?)
                })?;
            }
        }
    }
    Ok(())
}

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn clear(&self) -> Result<()> {
        debug!(self.inout, "Running clear action...");

        // check command and flags
        if self.args.params().len() > 1 {
            return self.invalid_args_err(1);
        }
        self.check_flags(
            "run",
            &[
                "--list",
                "-l",
                "--assume-yes",
                "-y",
                "--assume-no",
                "-n",
                "--no-color",
                "--debug",
            ],
        )?;

        // get args
        let wflag_list = self.args.flags().contains(&Flag::Word("list".into()));
        let lflag_list = self.args.flags().contains(&Flag::Letter('l'));
        let flag_list = wflag_list || lflag_list;

        // resolve profile into all leafs
        let profile = String::new();
        let mut profile_loader = self.profile_loader()?;
        let root_profile = profile_loader.load(&profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // paths
        let backup_dir = self.paths("backup")?;
        let run_dir = self.paths("run")?;

        // get all tracked paths
        let mut tracked_paths = HashSet::new();
        for profile in profiles {
            // load tracked paths
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(module) => {
                    let backup_dir = backup_dir.join(module.id());
                    let module = module.resolve(&backup_dir)?;
                    for entry in module.entries() {
                        tracked_paths.insert(backup_dir.join(entry.path()).canonicalize()?);
                    }
                }
                ProfileType::Runner(runner) => {
                    let run_dir = run_dir.join(runner.id());
                    let runner = runner.resolve(&run_dir)?;
                    for entry in runner.entries() {
                        tracked_paths.insert(run_dir.join(entry.path()).canonicalize()?);
                    }
                }
            };
        }

        // find untracked files outside profile related dirs
        let mut all_paths = self.paths("backup")?.all_files(AbsPath::FILTER_FILES)?;
        all_paths.extend(self.paths("run")?.all_files(AbsPath::FILTER_FILES)?);
        clear_paths(self, all_paths, &tracked_paths, flag_list)?;

        Ok(())
    }
}
