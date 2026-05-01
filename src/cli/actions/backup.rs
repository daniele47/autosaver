use crate::{
    cli::{actions::Runner, error::Result, flags::Flag, render::Renderer},
    core::profile::{ProfileType, composite::ProfileLoader},
};

impl<I: Renderer> Runner<I> {
    /// Backup action to list/save/restore files.
    pub fn backup(&mut self) -> Result<()> {
        let mut iter = self.args.params().iter();
        let arg_command = iter.next().map(String::as_str).unwrap_or_default();
        let arg_profile = iter.next().map(String::as_str).unwrap_or_default();
        let wflag_y = self.args.flags().contains(&Flag::Word("assumeyes".into()));
        let lflag_y = self.args.flags().contains(&Flag::Letter('y'));
        let flag_y = wflag_y || lflag_y;
        let wflag_n = self.args.flags().contains(&Flag::Word("assumeno".into()));
        let lflag_n = self.args.flags().contains(&Flag::Letter('n'));
        let flag_n = wflag_n || lflag_n;
        let wflag_diff = self.args.flags().contains(&Flag::Word("diff".into()));
        let lflag_diff = self.args.flags().contains(&Flag::Letter('d'));
        let flag_diff = wflag_diff || lflag_diff;
        let wflag_all = self.args.flags().contains(&Flag::Word("all".into()));
        let lflag_all = self.args.flags().contains(&Flag::Letter('a'));
        let flag_all = wflag_all || lflag_all;

        let home_dir = Self::paths("home");
        let backup_dir = Self::paths("backup");

        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load("test")?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(module) => {
                    let backup_dir = &backup_dir.joins(&[profile.name()]);
                    let module = module.merge_bases(&home_dir, &backup_dir)?;

                    // TODO: actions on all files
                    println!("{module:?} {arg_command} {flag_y} {flag_n}");
                    println!("{flag_diff} {flag_all} {root_profile:?} {arg_profile}");
                }
            }
        }

        todo!("Do operations on 1 module at a time")
    }
}
