use crate::{
    cli::{actions::Runner, error::Result, flags::Flag},
    core::profile::{ProfileType, composite::ProfileLoader},
};

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn clear(&mut self) -> Result<()> {
        // check command and flags
        if self.args.params().len() > 2 {
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
        let profile = self.load_profile(1)?;
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(&profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // iterate over all leaf profiles
        self.output_main_profile(&profile);
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(_) => {}
                ProfileType::Runner(_) => {
                    dbg!(&flag_list);
                    dbg!(&backup_dir);
                    dbg!(&run_dir);
                }
            }
        }
        Ok(())
    }
}
