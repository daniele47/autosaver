use std::collections::HashSet;

use crate::{
    cli::{actions::Runner, error::Result, flags::Flag},
    core::{
        fs::AbsPath,
        profile::{ProfileType, composite::ProfileLoader},
    },
    debug,
};

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn clear(&self) -> Result<()> {
        debug!(self.inout, "Running clear action...");

        // check command and flags
        if self.args.params().len() > 2 {
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
        let profile = self.load_profile(1)?;
        let mut profile_loader = self.profile_loader()?;
        let root_profile = profile_loader.load(&profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // paths
        let backup_dir = self.paths("backup")?;
        let run_dir = self.paths("run")?;

        // get all tracked paths
        self.output_main_profile(&profile);
        for profile in profiles {
            self.output_profile(profile.name());

            // load tracked paths
            let mut tracked_paths = HashSet::new();
            let dir = match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(module) => {
                    let backup_dir = backup_dir.join(module.id());
                    let module = module.resolve(&backup_dir)?;
                    for entry in module.entries() {
                        tracked_paths.insert(backup_dir.join(entry.path()).canonicalize()?);
                    }
                    backup_dir
                }
                ProfileType::Runner(runner) => {
                    let run_dir = run_dir.join(runner.id());
                    let runner = runner.resolve(&run_dir)?;
                    for entry in runner.entries() {
                        tracked_paths.insert(run_dir.join(entry.path()).canonicalize()?);
                    }
                    run_dir
                }
            };

            // act on all paths
            if let Ok(all_paths) = dir.all_files(AbsPath::FILTER_FILES) {
                for path in all_paths {
                    let canon = path.canonicalize()?;
                    let rel_path = canon.to_relative(&self.paths("root")?)?;
                    let rel_path_str = rel_path.to_str_lossy();
                    if !tracked_paths.contains(&canon) {
                        self.inout.writeln(rel_path_str, Self::PATH_UNTRACKED_COL);
                        if !flag_list {
                            self.prompt("Do you want to delete the untracked file?", |_| {
                                Ok(path.purge_path(false)?)
                            })?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
