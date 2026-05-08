use std::collections::HashSet;

use crate::{
    cli::{actions::Runner, error::Result, flags::Flag},
    core::{
        fs::AbsPath,
        profile::{ProfileType, composite::ProfileLoader},
    },
};

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn clear(&mut self) -> Result<()> {
        // check command and flags
        if self.args.params().len() > 1 {
            return self.invalid_cmd_err();
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
            ],
        )?;

        // get args
        let wflag_list = self.args.flags().contains(&Flag::Word("list".into()));
        let lflag_list = self.args.flags().contains(&Flag::Letter('l'));
        let flag_list = wflag_list || lflag_list;

        // paths
        let backup_dir = Self::paths("backup")?;
        let run_dir = Self::paths("run")?;

        // resolve profile into all leafs
        let profile = String::from(".");
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(&profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // get all tracked paths
        let mut tracked_paths = HashSet::<AbsPath>::new();
        self.output_main_profile("clear command");
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(module) => {
                    let backup_dir = backup_dir.joins(&[profile.name()]);
                    let module = module.resolve(&backup_dir)?;
                    for entry in module.entries() {
                        let abs_path = backup_dir.join(entry.path());
                        tracked_paths.insert(abs_path.canonicalize()?);
                    }
                }
                ProfileType::Runner(runner) => {
                    let run_dir = run_dir.joins(&[profile.name()]);
                    let runner = runner.resolve(&run_dir)?;
                    for entry in runner.entries() {
                        let abs_path = run_dir.join(entry.path());
                        tracked_paths.insert(abs_path.canonicalize()?);
                    }
                }
            }
        }

        // clear all paths in backup and run dir
        let mut all_paths = backup_dir.all_files(AbsPath::FILTER_FILES)?;
        all_paths.extend(run_dir.all_files(AbsPath::FILTER_FILES)?);
        for file in all_paths {
            let canon_path = file.canonicalize()?;
            let rel_path = file.to_relative(&Self::paths("root")?)?;
            let rel_path_str = rel_path.to_str_lossy();
            if !tracked_paths.contains(&canon_path) {
                self.inout.writeln(rel_path_str, Self::PATH_UNTRACKED_COL);
                if !flag_list {
                    self.prompt("Do you want to delete the untracked file?", |_| {
                        Ok(file.purge_path(false)?)
                    })?;
                }
            }
        }

        Ok(())
    }
}
